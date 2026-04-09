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

//
//! Entity types for the Mangater SDK.
//!
//! *See also:* Matcher trait definition in [`../traits/matcher.rs`]
//!

use crate::traits::{Config, Matcher, Storage, UrlFilter, UrlRewriter};

use std::collections::HashMap;
use std::sync::Arc;

use serde::Deserialize;
use serde_json::Value;

/// Represents a pattern and its associated type found on a web page.
///
/// `PatternAndType` combines a string-based pattern (typically a regular expression or substring)
/// with its corresponding [`PatternType`], allowing consumers to distinguish
/// between patterns for resources and pagination/navigation links.
pub struct PatternAndType {
    /// The pattern string (e.g., a regular expression or URL pattern) to match on the web page.
    pub pattern: String,
    /// The type of the pattern, specifying whether it is intended for a resource or pagination.
    pub pattern_type: PatternType,
    // [obsolete]
    // An extra selector to be used for the pattern matching.
    //pub extra_selector: Option<String>,
}

/// Represents the type of pattern matched on a web page.
///
/// `PatternType` helps specify whether the pattern is intended for a resource (such as image, PDF, video, audio, document, etc)
/// or for pagination/navigation purposes (such as the "next" page link).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternType {
    /// Used to match resource links on the page (e.g., images, pdfs, videos, audios, documents, etc).
    Resource,
    /// Used to match pagination links on the page (only focusing on the "next" pagination link).
    Pagination,
    /// Used to scrap the plain-text content (wikipedia crate might use this)
    Content,
    /// the content(s) are already ready for usage (e.g. storage or further analysis)
    ScrapedContent,
    /// provides the actual URI for fetching later on
    ActualUri,
    /// Used to match the uri of the pagination (e.g. "next" page link)
    /// other use cases... (to be defined or custom implementations can make use of this pattery-type)
    Others,
}

#[derive(Debug, Clone)]
pub struct PatternMatchResult {
    pub pattern: String,
    pub pattern_type: PatternType,
    pub resource_string: Option<String>,
    // for any additional parameters
    pub additoinal_params: Option<HashMap<String, String>>,
}

pub struct Registerable {
    /// Used to configure the domain/entity.
    /// Only if having custom implementations rather than env var OR config file(s).
    pub configurator: Option<Arc<dyn Config + Send + Sync>>,
    /// Used for content matching (i.e., parsing and identifying content on webpages).
    pub matcher: Arc<dyn Matcher + Send + Sync>,
    /// Used for persisting or caching results.
    pub storage: Option<Arc<dyn Storage + Send + Sync>>,
    /// Filters URLs relevant to the domain (optional).
    pub url_filter: Option<Arc<dyn UrlFilter + Send + Sync>>,
    /// Rewrites URLs to their canonical/resource form (optional).
    pub url_rewriter: Option<Arc<dyn UrlRewriter + Send + Sync>>,
}
// Box<dyn Domain>

#[derive(Debug, Clone)]
pub struct HtmlImage {
    pub src: String,
    pub inner_html: String,
}

#[derive(Debug, Clone)]
pub struct HtmlPlainTextAndImages {
    pub text: String,
    pub images: Vec<HtmlImage>,
}

#[derive(Debug, Deserialize)]
pub struct AppConfigJson5 {
    pub core: CoreConfig,

    #[serde(default)]
    pub plugins: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct CoreConfig {
    #[serde(default)]
    pub proxy: Option<ProxyConfig>,

    pub storage: StorageConfig,

    pub max_concurrency: u32,
}

#[derive(Debug, Deserialize)]
pub struct ProxyConfig {
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StorageConfig {
    #[serde(default)]
    pub root_folder: String,
}

/// Context object containing parameters for plugin execution.
///
/// `PluginContext` provides access to the key-value parameters supplied to a plugin at runtime.
/// These parameters can be used to guide plugin behavior depending on the specific use case.
#[derive(Clone, Debug, Default)]
pub struct PluginContext {
    params: HashMap<String, String>,
}

impl PluginContext {
    pub fn new(params: HashMap<String, String>) -> Self {
        Self { params }
    }

    // read
    pub fn get(&self, key: &str) -> Option<&String> {
        self.params.get(key)
    }

    // write
    pub fn insert(&mut self, key: String, value: String) {
        self.params.insert(key, value);
    }
}
