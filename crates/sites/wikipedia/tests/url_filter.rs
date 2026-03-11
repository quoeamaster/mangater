use mangater_sdk::traits::UrlFilter;
use site_wikipedia::WikipediaInstance;
use tracing_subscriber::EnvFilter;

fn init_tracing() {
    let filter = EnvFilter::new("info");

    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_file(true)
        .with_line_number(true)
        .try_init();
}

#[test]
fn test_wikipedia_url_filter() {
    init_tracing();
    struct TestCases {
        url: String,
        expected: bool,
    }

    let test_cases = vec![
        TestCases {
            url: "https://upload.wikimedia.org/wiki/Rust_(programming_language)/lang.png"
                .to_string(),
            expected: true,
        },
        TestCases {
            url: "https://en.wikipedia.org/wiki/Rust_(programming_language)".to_string(),
            expected: false,
        },
        TestCases {
            url: "https://www.wikipedia.org/wiki/Main_Page".to_string(),
            expected: false,
        },
        TestCases {
            url: "https://www.wikipedia.org/wiki/Main_Page?action=edit&section=123&foo=bar"
                .to_string(),
            expected: false,
        },
        TestCases {
            url: "https://upload.wikimedia.org/wiki/Rust_(programming_language)/lang.png?foo=bar"
                .to_string(),
            expected: true,
        },
    ];

    let wikipedia = WikipediaInstance::new();
    for test_case in test_cases {
        let filtered = wikipedia.filter_url(test_case.url.as_str());
        tracing::debug!("original url {}, filtered to {}", test_case.url, filtered);
        
        assert_eq!(
            filtered,
            test_case.expected,
            "expect a match on url {} for {} but end up opposite",
            test_case.url,
            test_case.expected
        );
    }
}
