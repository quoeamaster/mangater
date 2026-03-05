use crate::orchestration::model::RegistryMapImplementation;
use mangater_sdk::entity::AppConfigJson5;
use mangater_sdk::traits::Registry;
use mangater_sdk::SdkError;
use std::fs;

pub struct Engine {
    registry: RegistryMapImplementation,

    /// configuration (core and plugins)
    config: Option<AppConfigJson5>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            registry: RegistryMapImplementation::new(),
            config: None,
        }
    }

    /// return a read-only reference to the underlying registry implementation
    pub fn registry(&mut self) -> &mut dyn mangater_sdk::traits::Registry {
        &mut self.registry
    }

    pub fn config_load_from_json5_file(
        &mut self,
        config_file: String,
    ) -> Result<&AppConfigJson5, SdkError> {
        let config_content = fs::read_to_string(config_file.clone()).map_err(|e| {
            SdkError::InvalidConfig(format!("{} - {}", config_file.clone(), e.to_string()))
        })?;

        let config: AppConfigJson5 =
            json5::from_str(&config_content).map_err(|e| SdkError::InvalidConfig(e.to_string()))?;

        self.config = Some(config);

        Ok(self.config.as_ref().unwrap())
    }

    pub fn config_load_from_json_file(
        &mut self,
        config_file: String,
    ) -> Result<&AppConfigJson5, SdkError> {
        let config_content = fs::read_to_string(config_file.clone()).map_err(|e| {
            SdkError::InvalidConfig(format!("{} - {}", config_file.clone(), e.to_string()))
        })?;

        let config: AppConfigJson5 = serde_json::from_str(&config_content)
            .map_err(|e| SdkError::InvalidConfig(e.to_string()))?;

        self.config = Some(config);

        Ok(self.config.as_ref().unwrap())
    }
}

impl Engine {
    pub async fn run_scrap_workflow(&self, url: String) -> Result<(), SdkError> {
        let domain = self.registry.resolve_domain(url.as_str());
        // actually if no Domain found, not supported and throw an error
        if domain.is_none() {
            return Err(SdkError::Unsupported(url.to_string()));
        }
        if let Some(domain) = domain {
            let patterns = domain.get_domain_registerable().matcher.match_patterns();
            tracing::info!("patterns: {:?}", patterns);

            // next...
            // check the patterns and check if need to scrap OR the content already ready for storage...
        }
        Ok(())
    }
}
