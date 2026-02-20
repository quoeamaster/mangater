use crate::orchestration::model::RegistryMapImplementation;

pub struct Engine {
    registry: RegistryMapImplementation,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            registry: RegistryMapImplementation::new(),
        }
    }

    /// return a read-only reference to the underlying registry implementation
    pub fn registry(&mut self) -> &mut dyn mangater_sdk::traits::Registry {
        &mut self.registry
    }
}
