// mangater-sdk - the sdk interface for Mangater, includes traits, models and utilities.
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

//! traits/registry.rs provides the trait for registering domains.

use crate::traits::Domain;

use std::sync::Arc;

pub trait Registry {
    /// Register a domain under the given key. Replaces any existing entry with the same key (no duplicates per key).
    /// key could be the following:
    /// - domain name (e.g. "www.wikipedia.org")
    /// - plugin / implementation name (e.g. "wikipedia")
    fn add_to_registry(&mut self, key: Option<String>, domain: Arc<dyn Domain>);

    /// Resolve a domain from a given URL.
    ///
    /// # Arguments
    /// * `url`: The URL to resolve the domain from.
    ///
    /// # Returns
    /// * `(Option<Arc<dyn Domain>>, String)` - The domain and the key.
    /// - `None` if the domain is not found.
    /// - `Some(Arc<dyn Domain>)` if the domain is found.
    /// - `String` the key of the domain.
    fn resolve_domain(&self, url: &str) -> (Option<Arc<dyn Domain>>, String);

    /// List all registered domains.
    ///
    /// # Returns
    /// * `Vec<String>` - The list of registered domains.
    /// - `String` the key of the domain.
    fn list_registered_domains(&self) -> Vec<String>;
}
