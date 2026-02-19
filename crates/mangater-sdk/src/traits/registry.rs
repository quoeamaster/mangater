use crate::traits::Domain;

pub trait Registry {
    /// Register a domain under the given key. Replaces any existing entry with the same key (no duplicates per key).
    /// key could be the following:
    /// - domain name (e.g. "www.wikipedia.org")
    /// - plugin / implementation name (e.g. "wikipedia")
    /// as long as it can uniquely identify the implementation.
    fn add_to_registry(&mut self, key: Option<String>, domain: Box<dyn Domain>);
}
