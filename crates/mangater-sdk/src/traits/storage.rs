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

use crate::entity::PatternMatchResult;
use crate::errors::SdkError;

/// The `Storage` trait defines the interface for persisting resources extracted by the Mangater SDK.
///
/// Implementors are responsible for defining how and where the resource and its associated content bytes
/// are stored (e.g., filesystem, database, remote blob storage, etc.).
pub trait Storage: Send + Sync {
    /// Persists the given resource and its content.
    ///
    /// # Arguments
    ///
    /// * `resource` - A reference to the `PatternMatchResult` that describes the matched resource metadata.
    /// * `resource_content` - A vector of bytes representing the actual content (e.g., downloaded file, image, or document) to be stored.
    ///
    /// # Returns
    ///
    /// * `Result<(), SdkError>` - Returns `Ok(())` if the resource was successfully persisted, or an `SdkError` if an error occurred.
    fn persist(
        &self,
        resource: &PatternMatchResult,
        resource_content: Vec<u8>,
    ) -> Result<(), SdkError>;
}
