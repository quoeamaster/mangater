use mangater_sdk::entity::PatternType;
use mangater_sdk::entity::{PatternMatchResult, Registerable};
use mangater_sdk::traits::{Config, Domain, Matcher};
use mangater_sdk::SdkError;

use regex::Regex;

use once_cell::sync::Lazy;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

use crate::runner::model::WikipediaConfig;

/// for wikipedia domain matching, a static regex is used to avoid recompilation on each match.
static WIKI_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^https://([a-zA-Z0-9-]+\.)*wikipedia\.org(/.*)?$").unwrap());

static WIKI_DOMAIN_KEY: Lazy<String> = Lazy::new(|| "wikipedia".to_string());

#[derive(Clone, Debug)]
pub struct WikipediaInstance {
    pub domain_key: String,
    config: WikipediaConfig,
}

impl WikipediaInstance {
    pub fn new() -> Self {
        Self {
            domain_key: WIKI_DOMAIN_KEY.to_string(),
            config: WikipediaConfig::default(),
        }
    }
}

impl Domain for WikipediaInstance {
    fn match_domain(&self, domain: String) -> Result<bool, SdkError> {
        // easiest way... but not 100% accurate
        //Ok(domain.contains("wikipedia.org"))

        Ok(WIKI_REGEX.is_match(&domain))
    }

    // fn register_domain(
    //     &self,
    //     //registry: Box<dyn Registry>,
    //     domain: String,
    //     implementations: &Registerable,
    // ) {
    //     warn!("TBD: WikipediaInstance::register_domain");
    // }

    fn get_domain_key(&self) -> String {
        self.domain_key.clone()
    }

    fn get_domain_registerable(&self) -> Registerable {
        Registerable {
            configurator: None,
            matcher: Arc::new(self.clone()), // matcher: Arc::new(WikipediaInstance), (if stateless, no need to clone)
            storage: None,
        }
    }
}

impl Matcher for WikipediaInstance {
    /// for wikipedia domain, there could be 2 approaches to match and scrap.
    /// 1. provide a vector of PatternMatchResult for the engine to handle the rest.
    /// 2. totally override the operations by calling sdk's util functions to match and scrap;
    ///    then return a vector of PatternMatchResult for the engine to handle the rest. (type is `ScrapedContent`)
    ///
    /// for simplicity, will use approach 1 for now.
    fn match_patterns(&self) -> Vec<PatternMatchResult> {
        let mut results = Vec::new();

        // means scrap the images based on <img> tag
        results.push(PatternMatchResult {
            pattern: "img".to_string(),
            pattern_type: PatternType::Resource,
            resource_string: None,
        });
        // need to scrap the plain-text content???
        if self.config.need_content {
            results.push(PatternMatchResult {
                pattern: "#mw-content-text".to_string(),
                pattern_type: PatternType::Content,
                resource_string: None,
            });
        }
        results
    }
}

impl Config for WikipediaInstance {
    fn load(&mut self, raw_config_values: HashMap<String, Value>) -> Result<(), SdkError> {
        if let Some(config) = raw_config_values.get(self.domain_key.as_str()) {
            self.config = serde_json::from_value(config.clone())
                .map_err(|e| SdkError::InvalidConfig(e.to_string()))?;
            return Ok(());
        }
        Ok(())
    }
}

// for most cases, Config and Storage traits are not required as the default implementations are sufficient (provided by core::Engine)

// impl Config for WikipediaInstance {
//     fn load(&self) -> Result<Option<String>, SdkError> {
//         warn!("TBD: WikipediaInstance::load");
//         Ok(None)
//     }
//     fn config_by_key(&self, key: &str) -> Result<Option<String>, SdkError> {
//         warn!("TBD: WikipediaInstance::config_by_key");
//         Ok(None)
//     }
// }

// #[async_trait]
// impl Storage for WikipediaInstance {
//     async fn persist(
//         &self,
//         resource: &PatternMatchResult,
//         resource_content: Vec<u8>,
//     ) -> Result<(), SdkError> {
//         warn!("TBD: WikipediaInstance::persist");
//         Ok(())
//     }
// }
