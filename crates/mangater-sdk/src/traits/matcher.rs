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

//! traits/matcher.rs provides the trait for matching resources.

use crate::entity::PatternMatchResult;
use crate::entity::PluginContext;

/// The `Matcher` trait defines the interface for matching resource patterns from a given source,
/// such as a web page. Implementors should provide logic that takes a list of patterns (with associated types),
/// searches for matches according to their own matching logic, and returns the results as a collection of `PatternMatchResult`.
pub trait Matcher: Send + Sync {
    /// Match patterns from a given source.
    ///
    /// # Arguments
    /// * `url`: The URL to match patterns from.
    /// * `context`: The context of the matching.
    ///
    /// # Returns
    /// * `Vec<PatternMatchResult>` - The results of the matching
    fn match_patterns(
        &self,
        url: &str,
        context: Option<&mut PluginContext>,
    ) -> Vec<PatternMatchResult>;
}
