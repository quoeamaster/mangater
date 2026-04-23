# `site-mangadex`

Mangater site plugin for MangaDex (`mangadex.org` and the public at-home API host). The public API is a single type, `[MangadexInstance](src/runner/instance.rs)`, which wires the `mangater-sdk` `Domain` and `Matcher` traits. Unlike HTML-first plugins, matching resolves chapter page URLs into **concrete image URLs** by calling MangaDex’s at-home server API and emitting `[PatternType::ActualUri](../../mangater-sdk/src/entity/model.rs)` results for each page.

## Plugin structure

```
src/
├── lib.rs                 # re-exports MangadexInstance
├── runner.rs              # runner submodules: public instance, internal model + util
└── runner/
    ├── model.rs           # AtHomeResponse, ChapterData (serde, API JSON)
    ├── util.rs            # extract_chapter_id_from_url, fetch_image_urls (reqwest)
    └── instance.rs        # Domain, Matcher (MANGADEX_REGEX, registerable)
testdata/raw/              # example at-home API JSON (shape reference)
```

Note: `#[cfg(test)]` tests live in `instance.rs` (domain regex) and `util.rs` (chapter id parsing; optional ignored network smoke test).

## Flow (how the engine uses this plugin)

1. **Domain** — The engine tests whether a URL belongs to this plugin with `Domain::match_domain`. A small precompiled set of patterns (`MANGADEX_REGEX` in `instance.rs`) matches:
  - `https://mangadex.org/chapter/...` (chapter view URLs), and
  - `https://api.mangadex.org/at-home/server/...` (at-home server endpoint URLs).

```rust
static MANGADEX_REGEX: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"^https://mangadex\.org/chapter/(.*)?$").unwrap(),
        Regex::new(r"^https://api\.mangadex\.org/at-home/server/(.*)?$").unwrap(),
    ]
});
```

1. **Registration** — `Domain::get_domain_registerable` returns a `Registerable` (see `mangater-sdk`) with **only** the matcher set (`Arc` clone of `MangadexInstance`). `configurator`, `storage`, `url_filter`, and `url_rewriter` are all `None`: there is no per-site JSON `Config` trait, no custom URL filter, and no rewriter in this crate.
2. **Matching** — `Matcher::match_patterns` is the main behavior:
  - It derives a **chapter id** with `[extract_chapter_id_from_url](src/runner/util.rs)`, which looks for the segment after `"/chapter/"` in the URL. That path currently matches **chapter page URLs** such as `https://mangadex.org/chapter/{id}/...` (see unit tests in `util.rs`). URLs that only match the at-home `api.mangadex.org/.../server/...` regex in `match_domain` do not yield a chapter id via this helper, so they produce **no** `PatternMatchResult` entries even though the domain check passes.
  - For a resolved chapter id, it runs `[fetch_image_urls](src/runner/util.rs)` **synchronously** via `mangater_sdk::util::async_util::block_on_async` on the async at-home call. That function uses `reqwest` with a **curl-like user agent** and `http1_only()` (noted in code as important for MangaDex), `GET` `https://api.mangadex.org/at-home/server/{chapter_id}`, deserializes `[AtHomeResponse](src/runner/model.rs)`, and builds full image URLs as `{baseUrl}/data/{hash}/{filename}` from `chapter.data`. On any error or empty id, the matcher returns an empty vector; API failures are logged and treated as no patterns.
  - Each image becomes a `PatternMatchResult` with `pattern_type: PatternType::ActualUri`, a placeholder `pattern` string (`"user_agent:curl/7.88.1"`, aligned with the client identity used for the API), `resource_string` set to the image URL, and `additoinal_params` including `chapter_id` and `filepath` (the same URL again).

## Implemented traits (mangater-sdk)


| Trait (crate `mangater_sdk::traits`) | Methods implemented                                         | Role in this plugin                                                                          |
| ------------------------------------ | ----------------------------------------------------------- | -------------------------------------------------------------------------------------------- |
| `Domain`                             | `match_domain`, `get_domain_key`, `get_domain_registerable` | Recognize chapter and at-home API URL shapes; domain key `mangadex`; register matcher only.  |
| `Matcher`                            | `match_patterns`                                            | Resolve chapter id → at-home API → one `ActualUri` result per page image, with extra params. |


`MangadexInstance` implements `Default` and `Clone` (shared via `Arc` in `Registerable`). This crate does **not** implement `Config`, `UrlFilter`, or `UrlRewriter`. `Storage` and `Configurator` in `Registerable` are not used here.

## Tricky or notable implementation details

- **Precompiled regex list** — `MANGADEX_REGEX` uses `once_cell::sync::Lazy` so patterns are not recompiled on every `match_domain` call. Matching is case-sensitive; hosts such as `API.mangadex.org` or extra subdomains like `en.api...` do not match (see tests in `instance.rs`).
- **Async inside sync matcher** — `match_patterns` blocks on a Tokio/reqwest at-home request. The engine’s call path should tolerate this latency; long-running or failing network calls return no patterns and log a warning.
- **Chapter id extraction vs domain regex** — `extract_chapter_id_from_url` only understands the `/chapter/{id}` shape. `match_domain` also allows `api.mangadex.org/at-home/server/...`, but the matcher will not expand those URLs until `extract_chapter_id_from_url` is extended (or a separate code path is added) to parse the server URL form.
- **Full images only** — `fetch_image_urls` builds URLs from `chapter.data`, not from `dataSaver` (if present in JSON).
- **User agent** — The at-home client uses a fixed curl-style user agent and HTTP/1 only, matching MangaDex’s typical expectations for simple clients.
- **Plugin context** — `match_patterns` takes `Option<&mut PluginContext>` but does not use it; behavior does not depend on `PluginContext` (contrast with Wikipedia’s `scrap_content` usage).

