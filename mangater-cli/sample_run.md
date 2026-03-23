# sample run (use config root_folder)
cargo run -- scrap -l debug --url https://en.wikipedia.org/wiki/Relational_database -c testdata/config-local.json5

# use output folder override
cargo run -- scrap -l debug --url https://en.wikipedia.org/wiki/Relational_database -c testdata/config-local.json5 --output testdata/scrap-local-content

# mangadex example
cargo run -- scrap --url https://mangadex.org/chapter/5dec8fbf-243e-4c49-9213-11771294792b -c testdata/config-local.json5 --output testdata/scrap-local-content --param 'title=kinnikuman new' --param site=mangadex

## sample output on the params HashMap
# 2026-03-23T04:44:07.597225Z  INFO crates/mangater-core/src/orchestration/engine.rs:109: params: {"site": "mangadex", "title": "kinnikuman new"}
