use mangater_sdk::traits::Domain;
use mangater_sdk::traits::Registry;
use std::collections::HashMap;
use std::sync::Arc;

pub struct RegistryMapImplementation {
    pub registry: HashMap<String, Arc<dyn Domain>>,
}

impl RegistryMapImplementation {
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
        }
    }
}

impl Registry for RegistryMapImplementation {
    fn add_to_registry(&mut self, key: Option<String>, domain: Arc<dyn Domain>) {
        let new_key = match key {
            Some(k) => k,
            None => domain.get_domain_key(),
        };
        // this step should be done by the domain implementation itself (and not here probably)
        //domain.register_domain(new_key.clone(), domain.get_domain_registerable());
        self.registry.insert(new_key, domain);
    }

    fn resolve_domain(&self, url: &str) -> Option<Arc<dyn Domain>> {
        for domain in self.registry.values() {
            if let Ok(true) = domain.match_domain(url.to_string()) {
                return Some(Arc::clone(domain));
            }
        }
        None
    }

    fn list_registered_domains(&self) -> Vec<String> {
        self.registry.keys().cloned().collect()
    }
}


// flow on resolving a domain from a url

// URL
//  ↓
// Registry.resolve(url)
//  ↓
// Arc<dyn Domain>
//  ↓
// domain.get_domain_registerable()
//  ↓
// Arc<dyn Matcher>
//  ↓
// Scrape

// sample code usage after resolving a domain from a url

// if let Some(domain) = registry.resolve_domain(url) {
//     let registerable = domain.get_domain_registerable();

//     let matcher = registerable.matcher;

//     if matcher.matches(url) {
//         println!("Matched domain: {}", domain.get_domain_key());
//     }
// }
