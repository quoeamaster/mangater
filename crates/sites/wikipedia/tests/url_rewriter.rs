use mangater_sdk::traits::UrlRewriter;
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
fn test_wikipedia_url_rewriter() {
    init_tracing();
    struct TestCases {
        url: String,
        expected: String,
    }

    let test_cases = vec![
        TestCases {
            url: "https://upload.wikimedia.org/wikipedia/commons/thumb/4/41/Global_thinking.svg/20px-Global_thinking.svg.png".to_string(),
            expected: "https://upload.wikimedia.org/wikipedia/commons/4/41/Global_thinking.svg".to_string(),
        },
        TestCases {
            url: "https://upload.wikimedia.org/wikipedia/commons/thumb/4/41/Global_thinking.svg/20px-Global_thinking.svg.png?foo=bar".to_string(),
            expected: "https://upload.wikimedia.org/wikipedia/commons/4/41/Global_thinking.svg".to_string(),
        },
        TestCases {
            url: "https://upload.wikimedia.org/wikipedia/commons/thumb/99/999/db.svg/20px-db.svg.png?foo=bar".to_string(),
            expected: "https://upload.wikimedia.org/wikipedia/commons/99/999/db.svg".to_string(),
        },
    ];
    let wikipedia = WikipediaInstance::new();
    for test_case in test_cases {
        let rewritten = wikipedia.rewrite_url(test_case.url.as_str());
        tracing::debug!("original url {}, rewritten to {}", test_case.url, rewritten);

        assert_eq!(
            rewritten, test_case.expected,
            "original url {}, expected {}, but got {}",
            test_case.url, test_case.expected, rewritten
        );
    }
}
