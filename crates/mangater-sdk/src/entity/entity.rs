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

use crate::traits::{Config, Matcher, Storage};

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
}

pub struct PatternMatchResult {
    pub pattern: String,
    pub pattern_type: PatternType,
    pub resource_string: String,
}

/// Aggregates components required for registration within the Mangater SDK.
///
/// The `Registerable` struct acts as a container for the entities involved in the
/// configuration, matching, and storage functionalities. It is typically intended
/// for dynamic type usage, as each field is boxed and implements the respective
/// trait.
///
/// This is the heart of the SDK, as it combines everything required for a domain to work out.
/// - the configurator defines how config file(s) should be loaded
/// - the matcher defines how the content should be matched (resource level, navigation level etc)
/// - the storage defines how the content should be persisted (file, blob storage etc)
///
/// # Fields
///
/// - `configurator`: A boxed dynamic trait object implementing [`Config`] for configuration functionality.
/// - `matcher`: A boxed dynamic trait object implementing [`Matcher`] for pattern matching.
/// - `storage`: A boxed dynamic trait object implementing [`Storage`] for data persistence.
///
/// # Example
/// ```ignore
/// let reg = Registerable {
///     configurator: Box::new(MyConfigurator {}),
///     matcher: Box::new(MyMatcher {}),
///     storage: Box::new(MyStorage {}),
/// };
/// ```
pub struct Registerable {
    /// Used to configure the domain/entity.
    pub configurator: Box<dyn Config>,
    /// Used for content matching (i.e., parsing and identifying content on webpages).
    pub matcher: Box<dyn Matcher>,
    /// Used for persisting or caching results.
    pub storage: Box<dyn Storage>,
}
// Box<dyn Domain>
