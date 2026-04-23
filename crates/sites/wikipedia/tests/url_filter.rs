// wikipedia - the wikipedia site implementation for Mangater.
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
            filtered, test_case.expected,
            "expect a match on url {} for {} but end up opposite",
            test_case.url, test_case.expected
        );
    }
}
