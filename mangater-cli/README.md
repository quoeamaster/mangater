# mangater-cli

Command-line entry point for the Mangater stack. It loads a JSON/JSON5 config, registers **site plugins** based on [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html), and runs scrape workflows for URLs that match a registered plugin.

This document is a **on-going draft**: feature names and defaults match the crate’s `Cargo.toml` as of the current tree.

## Cargo features

Optional site crates are wired through feature flags. The `default` feature pulls in a bundle named `official-sites` (Wikipedia, Mangadex, and NASA search) when you do not override features.

| Cargo feature   | Enables crate          | Plugin / domain key (for `list-domains`) |
|-----------------|------------------------|------------------------------------------|
| `wikipedia`     | `site-wikipedia`       | `wikipedia`                              |
| `mangadex`      | `site-mangadex`        | `mangadex`                               |
| `nasa`          | `site-nasa-search`     | `nasa-search`                            |
| `official-sites`| `wikipedia`, `mangadex`, `nasa` | (same as enabling all three)   |

In `mangater-cli/Cargo.toml`, the feature block looks like this:

```toml
[features]
default = ["official-sites"]
official-sites = ["wikipedia", "mangadex", "nasa"]
wikipedia = ["site-wikipedia"]
mangadex = ["site-mangadex"]
nasa = ["site-nasa-search"]
```

To **trim the default build** (for example, Wikipedia only), set `default` to an empty list and keep only the features you need, or build with `default-features = false` (see below).

## Configuring `Cargo.toml`

### Consuming this crate as a dependency

Enable the site plugins you want. Without `default-features = false`, you get the crate’s `default` features (here, all of `official-sites`).

```toml
[dependencies]
mangater-cli = { path = "../mangater-cli" }
```

Wikipedia only:

```toml
[dependencies]
mangater-cli = { path = "../mangater-cli", default-features = false, features = ["wikipedia"] }
```

Wikipedia and Mangadex, but not NASA:

```toml
[dependencies]
mangater-cli = { path = "../mangater-cli", default-features = false, features = ["wikipedia", "mangadex"] }
```

### Building or running this package from the repo

From the workspace root, pass features to `cargo` so the binary is compiled with the right plugins:

```sh
# default (official-sites: wikipedia + mangadex + nasa)
cargo build -p mangater-cli
cargo run -p mangater-cli -- list-domains
```

```sh
# single plugin
cargo run -p mangater-cli --no-default-features --features wikipedia -- list-domains
```

```sh
# custom subset
cargo run -p mangater-cli --no-default-features --features "wikipedia,mangadex" -- scrap --url "https://en.wikipedia.org/wiki/NoSQL"
```

## CLI usage (draft examples)

Global options (apply to all subcommands):

- `-c` / `--config` — config file path (default `config.json5`)
- `--config-mode` — `json5` or `json` (default `json5`)
- `-l` / `--log-level` — `trace` | `debug` | `info` | `warn` | `error` (default `info`)

The executable produced by Cargo is **`mangater-cli`** (the package name). `clap` may show a different program name (e.g. `mangater`) in `--help` text.

### `list-domains`

Prints domain keys for every plugin that was **compiled in** and registered:

```sh
cargo run -p mangater-cli -- list-domains
# or, after `cargo build -p mangater-cli`:
./target/debug/mangater-cli list-domains
```

With a specific config file:

```sh
mangater-cli -c testdata/config.json5 list-domains
```

### `scrap`

Scraping runs when the URL matches a **matcher** for a registered plugin. Syntax:

```sh
mangater-cli scrap --url <URL> [--output <DIR>] [--param KEY=VALUE ...]
```

`-o` / `--output` overrides `core.storage.root_folder` from the config file (when implemented by the engine). `--param` is repeatable for plugin-specific parameters.

#### Wikipedia (`feature = "wikipedia"`)

URL must look like a Wikipedia page, e.g. `https://(<lang>.)?wikipedia.org/...`.

```sh
mangater-cli scrap --url "https://en.wikipedia.org/wiki/NoSQL" -c testdata/config.json5
```

The sample `testdata/config.json5` includes a `plugins.wikipedia` section (e.g. `need_content`).

#### Mangadex (`feature = "mangadex"`)

Typical chapter URLs (see the Mangadex site plugin for exact regexes), for example:

```sh
mangater-cli scrap --url "https://mangadex.org/chapter/<chapter-id>/1"
```

Replace `<chapter-id>` with a real chapter ID from a Mangadex URL you intend to scrape.

#### NASA Images API (`feature = "nasa"`)

The plugin matches URLs of the form `https://images-api.nasa.gov/search?q=...`. The search term is read from the query string. You can cap how many image results to process with `--param rows=N` (the plugin default is `10`).

```sh
mangater-cli scrap --url "https://images-api.nasa.gov/search?q=mars" --param rows=5
```

## Sample `config.json5`

```json
{
    // **** [core / engine sharable config] ****
    "core": {
        "proxy": {
            // "username": "",
            // "password": ""
        },
        "storage": {
            "root_folder": "/dev/null"
        },
        "max_concurrency": 8
    },

    // **** [plugin specific config] ****
    "plugins": {
        "wikipedia": {
            // **** [wikipedia - need to scrap plain-text content as well???] ****
            "need_content": true
        }
    }
}
```

## Notes

- If a feature is not enabled at compile time, its plugin is not registered: `list-domains` will not list it, and `scrap` will not handle matching URLs.
- This README is **draft** quality: URLs and behavior should be validated against the latest `mangater-cli` and site crates in your branch.
