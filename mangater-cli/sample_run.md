# sample run (use config root_folder)
cargo run -- scrap -l debug --url https://en.wikipedia.org/wiki/Relational_database -c testdata/config-local.json5

# use output folder override
cargo run -- scrap -l debug --url https://en.wikipedia.org/wiki/Relational_database -c testdata/config-local.json5 --output testdata/scrap-local-content
