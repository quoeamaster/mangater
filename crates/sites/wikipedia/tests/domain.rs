use mangater_sdk::traits::Domain;
use site_wikipedia::WikipediaInstance;

#[test]
fn test_wikipedia_domain_key() {
    let wikipedia = WikipediaInstance::new();
    assert_eq!(wikipedia.get_domain_key(), "wikipedia");
}

#[test]
fn test_wikipedia_domain_match() {
    let wikipedia = WikipediaInstance::new();
    assert!(wikipedia
        .match_domain("https://www.wikipedia.org/".to_string())
        .is_ok_and(|x| x));
    assert!(wikipedia
        .match_domain("https://jp.wikipedia.org/".to_string())
        .is_ok_and(|x| x));
    assert!(wikipedia
        .match_domain("https://en.wikipedia.org/".to_string())
        .is_ok_and(|x| x));
    assert!(wikipedia
        .match_domain("https://www.wikipedia.org/wiki/Main_Page".to_string())
        .is_ok_and(|x| x));
    assert!(wikipedia
        .match_domain("https://en.www.wikipedia.org/wiki/Main_Page?action=edit".to_string())
        .is_ok_and(|x| x));
    assert!(wikipedia
        .match_domain(
            "https://jp.www.wikipedia.org/wiki/Main_Page?action=edit&section=1".to_string()
        )
        .is_ok_and(|x| x));
    assert!(wikipedia
        .match_domain(
            "https://www.wikipedia.org/wiki/Main_Page?action=edit&section=123&foo=bar".to_string()
        )
        .is_ok_and(|x| x));

    // http is not supported (https OK)
    assert!(wikipedia
        .match_domain(
            "http://www.wikipedia.org/wiki/Main_Page?action=edit&section=123&foo=bar".to_string()
        )
        .is_ok_and(|x| !x));
}
