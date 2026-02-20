use mangater_cli::cmd;
use mangater_cli::util::engine::build_engine;

#[cfg(feature = "wikipedia")]
#[test]
fn test_engine_registry_list_domains() {
    let mut engine = build_engine();
    let domains = engine.registry().list_registered_domains();

    assert!(!domains.is_empty());
    assert_eq!(domains.contains(&"wikipedia".to_string()), true);
}

#[cfg(feature = "wikipedia")]
#[test]
fn test_cmd_list_domains() {
    let result = cmd::list_domains();
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
