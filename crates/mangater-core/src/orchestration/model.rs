use mangater_sdk::traits::Domain;

use std::collections::HashMap;

pub struct RegistryMapImplementation {
    pub registry: HashMap<String, Box<dyn Domain>>,
}

impl RegistryMapImplementation {
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
        }
    }
}

impl mangater_sdk::traits::Registry for RegistryMapImplementation {
    fn add_to_registry(&mut self, key: Option<String>, domain: Box<dyn Domain>) {
        let new_key = match key {
            Some(k) => k,
            None => domain.get_domain_key(),
        };
        // this step should be done by the domain implementation itself (and not here probably)
        //domain.register_domain(new_key.clone(), domain.get_domain_registerable());
        self.registry.insert(new_key, domain);
    }
}