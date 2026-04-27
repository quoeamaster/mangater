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

use mangater_sdk::traits::UrlRewriter;
use mangater_sites_wikipedia::WikipediaInstance;

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
