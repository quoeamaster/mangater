# `site-nasa-search`

Mangater site plugin for NASA’s Images API search host (`images-api.nasa.gov`). The public API is a single type, [`NasaSearchInstance`](src/runner/instance.rs), which wires the `mangater-sdk` domain and matcher to recognize search URLs, fetch the JSON response, and emit `PatternMatchResult` entries for large image assets.

## Plugin structure

```
src/
├── lib.rs                 # re-exports NasaSearchInstance
├── runner.rs              # `instance` and `model` submodules
└── runner/
    ├── model.rs           # NasaApiResponse, NasaCollection, NasaItem, NasaLink (serde, API JSON)
    └── instance.rs        # Domain, Matcher, NASA_SEARCH_REGEX, filter_images, tests
```

<!-- There is no top-level `tests/` directory; integration-style coverage lives in `instance.rs` under `#[cfg(test)]` (including an `#[ignore]`d API smoke test). -->

## Usage

1. **Add the crate** to your Mangater host (or another binary that links `mangater-sdk` plugins), e.g. in `Cargo.toml`:

   ```toml
   site-nasa-search = { path = "crates/sites/nasa-search" }
   ```

2. **Register the plugin** the same way as other site crates: use `NasaSearchInstance` as the `Domain` implementation; `get_domain_registerable` exposes only a **Matcher** (no configurator, storage, URL filter, or URL rewriter on the `Registerable`).

3. **URLs the plugin accepts** — `Domain::match_domain` returns true when the string matches:

```rust
static NASA_SEARCH_REGEX: Lazy<Regex> =
   Lazy::new(|| Regex::new(r"^https://images-api\.nasa\.gov/search\?q=(.*)$").unwrap());
```

   Example: `https://images-api.nasa.gov/search?q=mars` (query parameters beyond `q` may be present on the same URL as sent to the matcher; the implementation downloads the given `url` as returned by the engine).

4. **Plugin context** — `Matcher::match_patterns` requires `Some(&mut PluginContext)` and reads:

   - **`rows`** (optional) — how many image results to return after filtering; if missing, defaults to `"10"`. The value is parsed as `usize` for `take` on the filtered list.

5. **What the matcher returns** — For each selected asset, a `PatternMatchResult` with `PatternType::ActualUri`, `resource_string` and `additoinal_params["filepath"]` set to the same HTTPS link. Only links whose path contains `~large.jpg` are kept (see `filter_images` in `instance.rs`).

## Flow (how the engine uses this plugin)

1. **Domain** — `Domain::match_domain` uses a precompiled regex, `NASA_SEARCH_REGEX` (`once_cell::sync::Lazy<Regex>`), so the pattern is not recompiled on every call. It matches `https://` URLs whose host is `images-api.nasa.gov` and path/query start with `search?q=…`.
2. **Registration** — `Domain::get_domain_registerable` returns a `Registerable` with `matcher: Arc::new(self.clone())` and `configurator`, `storage`, `url_filter`, and `url_rewriter` all `None`. Domain key is the static string `nasa-search` (`NASA_SEARCH_DOMAIN_KEY`).
3. **Matching** — `Matcher::match_patterns`:
   - calls `block_on_async` and `download_resource` on the **incoming `url`** (the NASA search URL);
   - on failure, logs a warning and returns an empty `Vec`;
   - on success, deserializes the body as [`NasaApiResponse`](src/runner/model.rs) (JSON with `collection.items[]`, each with `links[].href`);
   - runs `filter_images` to collect `href` values containing `~large.jpg`;
   - takes at most `rows` (from context) results and maps each to a `PatternMatchResult`.

There is **no** `Config` trait implementation in this crate; there is no JSON config key under `nasa-search` like the Wikipedia plugin’s `WikipediaConfig`.

## Implemented traits (mangater-sdk)

| Trait (crate `mangater_sdk::traits`) | Methods implemented | Role in this plugin |
| --- | --- | --- |
| `Domain` | `match_domain`, `get_domain_key`, `get_domain_registerable` | Recognize `images-api.nasa.gov/search?q=…`; domain key `nasa-search`; publish matcher in `Registerable`. |
| `Matcher` | `match_patterns` | Download search URL, parse JSON, keep `~large.jpg` links, cap count with `rows`, emit `ActualUri` results. |

`NasaSearchInstance` implements `Default` and `Clone` (for `Arc` in `Registerable`). `Storage`, `UrlFilter`, `UrlRewriter`, and custom `Configurator` are not used in this crate’s `Registerable`.

## Tricky or notable implementation details

- **Single compiled regex for domains** — Same pattern as other site plugins: `NASA_SEARCH_REGEX` is shared across `match_domain` invocations.
- **Plugin context** — `match_patterns` uses `context.as_ref().unwrap()` for the `rows` key; a `None` context would panic. The default row count is the constant `DEFAULT_ROWS` (`"10"`); invalid `rows` strings would panic on `parse::<usize>()` (same as a malformed integer in context).
- **Response parsing** — Successful UTF-8 and JSON are assumed after download (`String::from_utf8` and `serde_json::from_str` use `unwrap`); unexpected API or encoding failures will panic rather than return empty results.
- **Filtering** — `filter_images` only keeps `~large.jpg` links (the commented-out variant also mentioned `~orig.tif`; the active code is `~large.jpg` only). Filtering is done inside the matcher, not via `UrlFilter`.
<!-- - **Tests** — `test_nasa_api_match_simulation` is marked `#[ignore]` and hits the live API; it uses a placeholder URL string in one call—when un-ignoring, align the URL with `NASA_SEARCH_REGEX` and a real `search?q=…` request. -->
