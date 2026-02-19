use mangater_sdk::traits::{ Domain, Config, Matcher, Storage };
use mangater_sdk::entity::{ Registerable, PatternAndType, PatternMatchResult };
use mangater_sdk::SdkError;

use tracing::warn;

pub struct WikipediaInstance {}

impl Config for WikipediaInstance {
    fn load(&self) -> Result<Option<String>, SdkError> {
        warn!("TBD: WikipediaInstance::load");
        Ok(None)
    }
    fn config_by_key(&self, key: &str) -> Result<Option<String>, SdkError> {
        warn!("TBD: WikipediaInstance::config_by_key");
        Ok(None)
    }
}

impl Domain for WikipediaInstance {
    fn match_domain(&self, domain: String) -> Result<bool, SdkError> {
        warn!("TBD: WikipediaInstance::match_domain");
        Ok(domain.contains("wikipedia.org"))
    }
    fn register_domain(
        &self,
        //registry: Box<dyn Registry>,
        domain: String,
        implementations: &Registerable,
    ) {
        warn!("TBD: WikipediaInstance::register_domain");
    }
    fn get_domain_key(&self) -> String {
        warn!("TBD: WikipediaInstance::get_domain_key");
        "wikipedia".to_string()
    }

    fn get_domain_registerable(&self) -> &Registerable {
        warn!("TBD: WikipediaInstance::get_domain_registerable");
        &Registerable {
            configurator: None,
            matcher: None,
            storage: None,
        }
    }
}

impl Matcher for WikipediaInstance {
    fn match_patterns(&self, patterns: &[PatternAndType]) -> Vec<PatternMatchResult> {
        warn!("TBD: WikipediaInstance::match_patterns");
        Vec::new()
    }
}

impl Storage for WikipediaInstance {
    fn persist(
        &self,
        resource: &PatternMatchResult,
        resource_content: Vec<u8>,
    ) -> Result<(), SdkError> {
        warn!("TBD: WikipediaInstance::persist");
        Ok(())
    }
}
