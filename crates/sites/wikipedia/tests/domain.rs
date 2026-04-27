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

use mangater_sdk::traits::Domain;
use mangater_sites_wikipedia::WikipediaInstance;

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
