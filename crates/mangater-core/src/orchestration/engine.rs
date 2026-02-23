use crate::orchestration::model::AppConfigJson5;
use crate::orchestration::model::RegistryMapImplementation;
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
