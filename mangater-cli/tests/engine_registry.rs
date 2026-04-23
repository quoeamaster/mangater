// mangater-cli - the CLI for Mangater
// Copyright (C) 2026 Takara-Mono <quoeamaster@gmail.com>
//
// For a copy of the MIT license, see <https://opensource.org/licenses/MIT>.
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

use mangater_cli::cmd;
use mangater_cli::entity::ConfigMode;
use mangater_cli::util::cli_engine::build_engine;

#[cfg(feature = "wikipedia")]
#[test]
fn test_engine_registry_list_domains() {
    let mut engine = build_engine(ConfigMode::Json5, Some("testdata/config.json5".to_string()));
    let domains = engine.registry().list_registered_domains();

    assert!(!domains.is_empty());
    assert_eq!(domains.contains(&"wikipedia".to_string()), true);
}

#[cfg(feature = "wikipedia")]
#[test]
fn test_cmd_list_domains() {
    let result = cmd::list_domains(ConfigMode::Json5, Some("testdata/config.json5".to_string()));
    assert!(result.is_ok());

    let domains = result.unwrap();
    assert!(!domains.is_empty());
    assert_eq!(domains.contains(&"wikipedia".to_string()), true);
}

// this test depends on how Cargo.toml was configured
//
// [features]
// default = ["official-sites"]
// # official-sites = ["wikipedia"]
// official-sites = []
// wikipedia = ["site-wikipedia"]
//
// then this test will be run with empty domains list...
//
// if official-sites = ["wikipedia"] instead...
// then this test will be run with `wikipedia` domain in the list...
//
// #[test]
// fn test_cmd_list_domains_without_wikipedia_features() {
//     let result = cmd::list_domains();
//     assert!(result.is_ok());

//     let domains = result.unwrap();
//     assert_eq!(domains.is_empty(), true, "Expected empty domains list, but got {domains:?}");
// }

// similar test for the case where wikipedia feature is not enabled...
// for ...
// [features]
// default = ["official-sites"]
// official-sites = ["wikipedia"]
// wikipedia = ["site-wikipedia"]
//
// this test will not be run at all as the wikipedia feature is not enabled...
//
// #[cfg(not(feature = "wikipedia"))]
// #[test]
// fn test_cmd_list_domains_no_wikipedia() {
//     let result = cmd::list_domains();
//     assert!(result.is_ok());

//     let domains = result.unwrap();
//     assert!(domains.is_empty());
// }
