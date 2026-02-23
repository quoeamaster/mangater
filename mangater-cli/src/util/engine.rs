use mangater_core::orchestration::Engine;

use std::sync::Arc;

use crate::entity::ConfigMode;

pub fn build_engine(
    config_mode: ConfigMode,
    config_file: Option<String>,
) -> mangater_core::orchestration::Engine {
    let mut engine = Engine::new();

    // load config from json5 file
    let config_file_path = config_file.unwrap_or("config.json5".to_string());
    let app_config = match config_mode {
        ConfigMode::Json5 => engine
            .config_load_from_json5_file(config_file_path)
            .unwrap(),
        ConfigMode::Json => engine.config_load_from_json_file(config_file_path).unwrap(),
    };
    tracing::debug!("overall app config: {:?}", app_config);

    #[cfg(feature = "wikipedia")]
    {
        use mangater_sdk::traits::{Config, Domain};
        use site_wikipedia::WikipediaInstance;

        let mut wikipedia = WikipediaInstance::new();
        // run config pre-load
        wikipedia.load(app_config.plugins.clone()).unwrap();

        engine.registry().add_to_registry(
            Some(wikipedia.get_domain_key()),
            Arc::new(wikipedia.clone()),
        );
    }
    //engine.registry().add_to_registry(None, Box::new(wikipedia::Wikipedia::new()));

    engine
}
