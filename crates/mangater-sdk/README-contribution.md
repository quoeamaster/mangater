# Contributing: scraping plugins for Mangater (mangater-sdk)

This document extends [README.md](./README.md) for anyone implementing a **site plugin** (scraping / resource extraction) on top of `mangater-sdk`. It summarizes what the SDK is for, which pieces you implement, and what you are expected to know as a developer.

## What the SDK is (from README)

- **mangater-sdk** is the shared interface (traits, models, utilities) that lets the Mangater ecosystem wire **how** to: recognize a site, match resource links, download or persist them, and produce a scrap summary.
- **Mangater is not a general web crawler.** It is oriented around a **logical page, chapter, or unit**: finding resources and (when needed) the next page *within* that boundary, not indiscriminate off-site crawling.

For two common site shapes, see [Site layouts and traits](#site-layouts-and-traits) below; they mirror the [README.md](./README.md) scenarios with names aligned to this crate’s actual APIs.

---

## What you need to know as a plugin author

### Rust and async

- **Rust (2021 edition)** and idiomatic trait objects where the host passes `Arc<dyn …>`.
- **`async_trait`**: `Storage` is `async` (`persist`). If your matcher is synchronous but you need to await I/O, you may use helpers such as `util::async_util` (or integrate with the host’s runtime as your binary defines). Check the utility fn `block_on_async` as well under the `/util/async_util.rs`.
- **Threading bounds**: core traits are `Send + Sync` so implementations can be shared across tasks.

### Web and content

- **HTTP**: how the target site serves images, documents, or APIs; redirects; optional cookies or auth (your plugin or host may use `reqwest` as in this crate’s [dependencies](./Cargo.toml)).
- **HTML structure**: how the gallery or reader HTML is laid out, stable selectors, and when the site **changes** markup (you will need to update patterns).
- **URL handling**: absolute vs relative URLs, query strings, and (if needed) canonicalization—`UrlRewriter` and `UrlFilter` exist for that pipeline.
- **Pattern design**: the matcher works from **string patterns** and `PatternType` (see [entity::model](src/entity/model.rs))—plan regexes, path prefixes, or conventions that separate **resource** links from **pagination** / **next** links.

### Data and errors

- **`serde_json::Value`**: the optional `Config` trait loads from `HashMap<String, Value>` so plugin-level configuration can be merged with app config.
- **`SdkError`**: the cross-layer error type for config, network, parse, not found, rate limits, auth, site-specific failures, and I/O—return or map into these variants so the host can surface consistent messages ([errors.rs](src/errors.rs)).

### Ecosystem (optional but common)

- **`scraper` / CSS selectors** and/or string parsing: the SDK provides HTML helpers under `mangater_sdk::util::html_parsing` (built on `scraper`).
- **`tracing`**: instrumenting your matcher or storage for debugging in production-style runs.

You do **not** need to implement the **`Registry`** trait in every plugin: that is the host’s table of `Domain` implementations. You **do** provide a `Domain` implementation that returns a `Registerable` so the host can register your plugin.

---

## Core types and registration

### `Registerable`

A plugin bundles its behavior in [`Registerable`](src/entity/model.rs):

| Field | Role |
|--------|------|
| `configurator` | Optional. Custom `Config` (e.g. not only env/file—database, remote config). |
| `matcher` | **Required** (Arc). `Matcher::match_patterns` drives discovery on a page. |
| `storage` | Optional. Where bytes go after fetch (`Storage::persist`). |
| `url_filter` | Optional. Exclude noise URLs. |
| `url_rewriter` | Optional. Normalize or expand URLs before fetch. |

### `Domain`

Implement [`Domain`](src/traits/domain.rs) to:

- Expose a stable key (`get_domain_key`).
- Report whether a hostname is yours (`match_domain` → `Result<bool, SdkError>`).
- Return the `Registerable` for this site (`get_domain_registerable`).

### `PluginContext`

[`PluginContext`](src/entity/model.rs) is an optional key/value bag passed into `Matcher::match_patterns` so the engine can pass runtime hints (e.g. mode flags). Use `get` / `insert` for parameters.

### `PatternType` and `PatternMatchResult`

- Define what you are matching with [`PatternAndType`](src/entity/model.rs) (`pattern` + `pattern_type`).
- Return [`PatternMatchResult`](src/entity/model.rs) with `resource_string` and optional `additoinal_params` (note the field name as in source) for downstream storage or display.

`PatternType` includes `Resource`, `Pagination` (e.g. “next” link), `Content`, `ScrapedContent`, `ActualUri`, and `Others`—pick the kinds that match how your site exposes assets and navigation.

---

## Site layouts and traits

These correspond to the README’s gallery vs. paginated flows, using **actual** trait and method names.

### a) Gallery-style: everything on one landing page

- All target assets (images, PDFs, etc.) are linked on one page; **no** “next page” required for a full set.
- **Typical focus**: `Resource` (and related) patterns in `match_patterns`.
- **Traits**: `Domain` + `Matcher` + `Storage` (and optional `Config`, `UrlFilter`, `UrlRewriter`).

### b) One resource per page with “next” navigation

- One primary resource per page; the “next” URL appears on the page. Still **within** a chapter/unit, not unbounded crawling.
- **Typical focus**: `Resource` plus **`Pagination`** (or `Others` as appropriate) so the engine can follow the next in-sequence link.
- **Same trait set** as (a); behavior differs in **which** `PatternType` values you emit and how the host’s loop uses them.

---

## Traits reference (quick)

| Trait | File | Purpose |
|-------|------|--------|
| `Domain` | [traits/domain.rs](src/traits/domain.rs) | Is this our site? Expose `Registerable`. |
| `Matcher` | [traits/matcher.rs](src/traits/matcher.rs) | `match_patterns(url, context)` → `Vec<PatternMatchResult>`. |
| `Storage` | [traits/storage.rs](src/traits/storage.rs) | `async fn persist(&resource, bytes)`. |
| `Config` | [traits/config.rs](src/traits/config.rs) | Optional. `load(raw_config_values)` for custom config sources. |
| `UrlFilter` | [traits/url_filter.rs](src/traits/url_filter.rs) | `filter_url(url) -> bool`. |
| `UrlRewriter` | [traits/url_rewriter.rs](src/traits/url_rewriter.rs) | `rewrite_url(url) -> String`. |
| `Registry` | [traits/registry.rs](src/traits/registry.rs) | Host-side registry, not the plugin’s core focus. |

---

## SDK utilities you can reuse

Under `mangater_sdk::util` ([util.rs](src/util.rs)):

- **`html_parsing`**: HTML cleaning, selectors, and related helpers.
- **`resource`**: download helpers (e.g. `download_resource`, file writes) with `SdkError` mapping.
- **`async_util`**: bridging async work where a synchronous boundary exists.

Use these to avoid duplicating fetch and parse logic; you can still add site-specific code in your crate.

---

## Suggested implementation checklist

1. Model the target site: one-shot list vs. paginated chain; list stable URL or DOM cues.
2. Implement `Domain` (match hostnames, return `Registerable`).
3. Implement `Matcher` using `PatternAndType` / `PatternType` and return rich `PatternMatchResult` rows.
4. Implement or delegate `Storage` to persist `Vec<u8>` the host passes after download.
5. Add `Config` / `UrlFilter` / `UrlRewriter` only if the site or deployment needs them.
6. Return `SdkError` from fallible operations that should surface to users or logs.

For the product-level overview, start with [README.md](./README.md). For the exact type definitions, use the [source](src/lib.rs) as the source of truth.
