use crate::runner::util::{block_on_async, extract_chapter_id_from_url, fetch_image_urls};

use mangater_sdk::entity::{PatternMatchResult, PatternType, PluginContext, Registerable};
use mangater_sdk::traits::{Domain, Matcher};
use mangater_sdk::SdkError;

use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::sync::Arc;

/// for managdex domain matching, a set of static regex is used to avoid recompilation on each match.
/// eg. https://mangadex.org/chapter/{chapter_id}/1
/// eg. https://api.mangadex.org/at-home/server/{chapter_id}
static MANGADEX_REGEX: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"^https://mangadex\.org/chapter/(.*)?$").unwrap(),
        Regex::new(r"^https://api\.mangadex\.org/at-home/server/(.*)?$").unwrap(),
    ]
});

static MANGADEX_DOMAIN_KEY: Lazy<String> = Lazy::new(|| "mangadex".to_string());

#[derive(Clone, Debug)]
pub struct MangadexInstance {
    pub domain_key: String,
}

impl Default for MangadexInstance {
    fn default() -> Self {
        Self::new()
    }
}

impl MangadexInstance {
    pub fn new() -> Self {
        Self {
            domain_key: MANGADEX_DOMAIN_KEY.to_string(),
        }
    }
}

impl Domain for MangadexInstance {
    fn match_domain(&self, domain: String) -> Result<bool, SdkError> {
        let mut matched = false;
        for regex in MANGADEX_REGEX.iter() {
            if regex.is_match(&domain) {
                matched = true;
                break;
            }
        }
        Ok(matched)
    }

    fn get_domain_key(&self) -> String {
        self.domain_key.clone()
    }

    fn get_domain_registerable(&self) -> Registerable {
        Registerable {
            configurator: None,
            matcher: Arc::new(self.clone()), // matcher: Arc::new(WikipediaInstance), (if stateless, no need to clone)
            storage: None,
            url_filter: None,
            url_rewriter: None,
        }
    }
}

impl Matcher for MangadexInstance {
    fn match_patterns(
        &self,
        url: &str,
        _context: Option<PluginContext>,
    ) -> Vec<PatternMatchResult> {
        // get chapter id from the url
        let chapter_id = match extract_chapter_id_from_url(url) {
            Some(id) => id,
            None => return vec![],
        };
        tracing::debug!("{} ->chapter id: {}", url, chapter_id);

        let image_urls = block_on_async(async {
            match fetch_image_urls(&chapter_id).await {
                Ok(urls) => urls,
                Err(e) => {
                    tracing::warn!(
                        "{} -> error fetching images: {:?}, return with empty vec![]",
                        url,
                        e
                    );
                    vec![]
                }
            }
        });
        image_urls
            .into_iter()
            .map(|u| PatternMatchResult {
                pattern: "user_agent:curl/7.88.1".to_string(),
                pattern_type: PatternType::ActualUri,
                resource_string: Some(u.clone()),
                additoinal_params: Some(HashMap::from([
                    ("chapter_id".to_string(), chapter_id.clone()),
                    ("filepath".to_string(), u.clone()),
                ])),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mangadex_regex() {
        struct TestCase {
            url: String,
            expected: bool,
        }
        let test_cases = vec![
            TestCase {
                url: "https://mangadex.org/chapter/1234567890".to_string(),
                expected: true,
            },
            TestCase {
                url: "https://api.mangadex.org/at-home/server/1234567890".to_string(),
                expected: true,
            },
            TestCase {
                url: "https://en.api.mangadex.org/at-home/server/1234567890".to_string(),
                expected: false,
            },
            TestCase {
                url: "https://APIs.mangadex.org/at-home/server/1234567890".to_string(),
                expected: false,
            },
            TestCase {
                url: "https://API.mangadex.org/at-home/server/1234567890".to_string(),
                // not ok, since upper/lower case not matched...
                expected: false,
            },
            TestCase {
                url: "https://jp.mangadex.org/chapter/1234567890".to_string(),
                expected: false,
            },
            TestCase {
                url: "https://mangadex.org.invalid/chapter/1234567890".to_string(),
                expected: false,
            },
            TestCase {
                url: "https://mangadex.org/chapterS/1234567890".to_string(),
                expected: false,
            },
        ];

        for target_test_case in test_cases {
            let mut matched = false;
            for regex in MANGADEX_REGEX.iter() {
                if regex.is_match(&target_test_case.url) {
                    matched = true;
                    break;
                }
            }
            assert_eq!(
                matched, target_test_case.expected,
                "url: {}, expected: {} actual value is {}",
                target_test_case.url, target_test_case.expected, matched
            );
        }
        // assert!(MANGADEX_REGEX[0].is_match("https://mangadex.org/chapter/1234567890"));
        // assert!(MANGADEX_REGEX[1].is_match("https://api.mangadex.org/at-home/server/1234567890"));
    }
}
