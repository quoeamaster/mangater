// mangater-sdk - the interface for Mangater
// Copyright (C) 2026 Takara-Mono <quoeamaster@gmail.com>
//
// This file is dual-licensed under the terms of the MIT.
//
// You may choose either license at your option. 
// For a copy of the MIT license, see <https://opensource.org/licenses/MIT>.
//
// MIT License:
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::errors::SdkError;
use crate::entity::Registerable;
use crate::traits::Registry;

/// The `Domain` trait provides an interface for matching a given domain string
/// to determine if it is supported or recognized by an implementation.
/// This is typically used to check if a scrapper or resource handler
/// can process content from a specific domain.
pub trait Domain {
    /// Determines if the provided domain string is supported by this implementation.
    ///
    /// # Parameters
    /// - `domain`: A string representing the domain to check for support (e.g., "www.example.com").
    ///
    /// # Returns
    /// - `Ok(true)` if the domain is recognized and supported.
    /// - `Ok(false)` if the domain is not recognized or unsupported.
    /// - `Err(SdkError)` if an error occurs during the matching process.
    fn match_domain(&self, domain: String) -> Result<bool, SdkError>;

    /// Registers a domain along with its associated trait implementations in the given registry.
    ///
    /// # Parameters
    /// - `registry`: A boxed trait object reference to a [`Registry`] where the domain mapping will be added.
    /// - `domain`: The domain string (e.g., `"www.example.com"`) to be registered.
    /// - `implementations`: A reference to a [`Registerable`] that groups the implementations for [`Config`], [`Matcher`], and [`Storage`] traits for this domain.
    ///
    /// # Usage
    /// Typically used during initialization to bind a supported website's functionality group to a domain key.
    ///
    /// # Example
    /// ```
    /// // Suppose `my_registry` implements Registry, and `registerable_impl` is a Registerable.
    /// domain.register_domain(Box::new(my_registry), "www.example.com".to_string(), &registerable_impl);
    /// ```
    fn register_domain(&self, registry: Box<dyn Registry>, domain: String, implementations: &Registerable);
}