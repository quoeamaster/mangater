# `site-wikipedia`

Mangater site plugin for `*.wikipedia.org`. The public API is a single type, [`WikipediaInstance`](src/runner/instance.rs), which wires the `mangater-sdk` domain, matcher, config, and URL hooks for Wikipedia.

## Plugin structure

```
src/
├── lib.rs                 # re-exports WikipediaInstance
├── runner.rs              # importing and re-exporing functions, struct, traits that is publicly accessible
└── runner/
    ├── model.rs           # WikipediaConfig (serde)
    └── instance.rs        # Domain, Matcher, Config, UrlFilter, UrlRewriter    
tests/                     # domain, url_filter, url_rewriter
```

## Flow (how the engine uses this plugin)

1. **Domain** — The engine tests whether a URL is a Wikipedia site using `Domain::match_domain`. A precompiled regex matches `https://` URLs whose host is `wikipedia.org` or a subdomain such as `en.wikipedia.org` (see `WIKI_REGEX` in `instance.rs`).
```rust
static WIKI_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^https://([a-zA-Z0-9-]+\.)*wikipedia\.org(/.*)?$").unwrap());
```
2. **Registration** — `Domain::get_domain_registerable` returns a `Registerable` (see `mangater-sdk`) that points the same `WikipediaInstance` (via `Arc`) at **Matcher**, **UrlFilter**, and **UrlRewriter**. Configurator and storage are left unset (`None`); configuration is still applied through the `Config` trait on the same type when the host loads raw JSON.
3. **Config** — If the global config map contains a key equal to the domain key (`"wikipedia"`), `Config::load` deserializes that JSON value into [`WikipediaConfig`](src/runner/model.rs) (currently the config contains only 1 field `need_content: bool`).
4. **Matching** — For matching URLs, `Matcher::match_patterns` tells the engine to:
   - set plugin context `scrap_content` to `"true"` (so source HTML is available for image scraping);
   - always scrape `<img>` resources;
   - optionally scrape main article text via the `#mw-content-text` selector when `need_content` is true.
5. **URL filter** — For resource URLs, `UrlFilter::filter_url` returns true only for `upload.wikimedia.org` (image/media assets on Wikimedia’s upload hosts).
6. **URL rewrite** — Thumbnail URLs under `/thumb/` are rewritten to the canonical file URL (strip the thumb path and size segment) via `UrlRewriter::rewrite_url`.

## Implemented traits (mangater-sdk)

| Trait (crate `mangater_sdk::traits`) | Methods implemented | Role in this plugin |
| --- | --- | --- |
| `Domain` | `match_domain`, `get_domain_key`, `get_domain_registerable` | Recognize Wikipedia hosts; domain key `wikipedia`; publish matcher + URL helpers in `Registerable`. |
| `Matcher` | `match_patterns` | Declares `img` + optional `#mw-content-text`; sets `scrap_content` in `PluginContext`. |
| `Config` | `load` | Merges JSON under key `wikipedia` into `WikipediaConfig`. |
| `UrlFilter` | `filter_url` | Keep URLs that belong to `upload.wikimedia.org`. |
| `UrlRewriter` | `rewrite_url` | Convert `/thumb/.../size-...` upload URLs to full-size asset paths. |

`WikipediaInstance` also implements `Default` and `Clone` (required so the same logic can be shared across `Arc` in `Registerable`). `Storage` in `Registerable` is not used here. The custom `Configurator` is used in the `mangater-cli` host program during the engine initialization stage.

## Tricky or notable implementation details

- **Single compiled regex for domains** — `WIKI_REGEX` is a `once_cell::sync::Lazy<Regex>` so the pattern is not recompiled on every `match_domain` call.
- **Plugin context** — `match_patterns` uses `context.unwrap()` and always inserts `scrap_content` → `"true"`. The engine is expected to pass `Some(PluginContext)`; a `None` context would panic.
- **Thumb URL shape** — `rewrite_url` splits on `/thumb/`, then requires at least four path segments after the split (`hash1`, `hash2`, `filename`, plus the rest). Otherwise the original URL is returned unchanged. This matches Wikimedia’s thumb URL layout, not arbitrary URLs.
- **Filter is substring-based** — `filter_url` uses `url.contains("upload.wikimedia.org")` rather than strict URL parsing; fast and sufficient for the tests and typical hrefs, but it is not a full URL structural check.
- **Shared `Arc` clones** — `get_domain_registerable` uses `Arc::new(self.clone())` for matcher, filter, and rewriter so one configuration-bearing instance is reused for all three roles.
