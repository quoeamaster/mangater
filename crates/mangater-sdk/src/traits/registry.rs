use crate::traits::Domain;

use std::sync::Arc;

pub trait Registry {
    /// Register a domain under the given key. Replaces any existing entry with the same key (no duplicates per key).
    /// key could be the following:
    /// - domain name (e.g. "www.wikipedia.org")
    /// - plugin / implementation name (e.g. "wikipedia")
    /// as long as it can uniquely identify the implementation.
    fn add_to_registry(&mut self, key: Option<String>, domain: Arc<dyn Domain>);

    fn resolve_domain(&self, url: &str) -> Option<Arc<dyn Domain>>;

    fn list_registered_domains(&self) -> Vec<String>;
}
