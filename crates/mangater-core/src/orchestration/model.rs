// mangater-core - the core utilities, several traits implementations for Mangater ecosystem.
// Copyright (C) 2026 Takara-Mono <quoeamaster@gmail.com>
//
// For a copy of the MIT license, see <https://opensource.org/licenses/MIT>.
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

//! orchestration/model.rs provides the model for the orchestration engine.

use mangater_sdk::traits::Domain;
use mangater_sdk::traits::Registry;
use std::collections::HashMap;
use std::sync::Arc;

/// RegistryMapImplementation is a struct that implements the [Registry](mangater_sdk::traits::Registry) trait
/// and utilizes a [HashMap](std::collections::HashMap) to store the domain(s).
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

    fn resolve_domain(&self, url: &str) -> (Option<Arc<dyn Domain>>, String) {
        for domain_key in self.registry.keys() {
            if let Some(domain) = self.registry.get(domain_key) {
                if let Ok(true) = domain.match_domain(url.to_string()) {
                    return (Some(Arc::clone(domain)), domain_key.clone());
                }
            }
        }
        (None, "".to_string())
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
