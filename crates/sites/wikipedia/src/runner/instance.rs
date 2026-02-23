use mangater_sdk::entity::{PatternAndType, PatternMatchResult, Registerable};
use mangater_sdk::traits::{Domain, Matcher};
use mangater_sdk::SdkError;

use regex::Regex;
use async_trait::async_trait;
use tracing::warn;

use once_cell::sync::Lazy;
use std::sync::Arc;

/// for wikipedia domain matching, a static regex is used to avoid recompilation on each match.
static WIKI_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^https://([a-zA-Z0-9-]+\.)*wikipedia\.org(/.*)?$").unwrap());

#[derive(Clone, Debug)]
pub struct WikipediaInstance {
    pub domain_key: String,
}

impl WikipediaInstance {
    pub fn new() -> Self {
        Self {
            domain_key: "wikipedia".to_string(),
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

#[async_trait]
impl Matcher for WikipediaInstance {
    async fn match_patterns(&self, patterns: &[PatternAndType]) -> Vec<PatternMatchResult> {
        warn!("TBD: WikipediaInstance::match_patterns");
        Vec::new()
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
