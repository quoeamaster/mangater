use mangater_core::orchestration::Engine;

use std::sync::Arc;

pub fn build_engine() -> mangater_core::orchestration::Engine {
    let mut engine = Engine::new();

    #[cfg(feature = "wikipedia")]
    {
        use mangater_sdk::traits::Domain;
        use site_wikipedia::WikipediaInstance;

        let wikipedia = WikipediaInstance::new();
        // run config pre-load

        engine.registry().add_to_registry(
            Some(wikipedia.get_domain_key()),
            Arc::new(wikipedia.clone()),
        );
    }
    //engine.registry().add_to_registry(None, Box::new(wikipedia::Wikipedia::new()));

    engine
}
