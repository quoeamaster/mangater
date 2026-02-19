use mangater_core::orchestration::Engine;

pub fn build_engine() -> mangater_core::orchestration::Engine {
    let engine = Engine::new();

    //engine.registry().add_to_registry(None, Box::new(wikipedia::Wikipedia::new()));

    engine
}



