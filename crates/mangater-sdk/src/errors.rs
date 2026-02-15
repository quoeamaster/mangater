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

use thiserror::Error;

/// Canonical error type exposed by mangater SDK.
///
/// This defines the error contract between:
/// - Core engine
/// - Site plugins
/// - CLI
#[derive(Debug, Error)]
pub enum SdkError {
    /// Network-level failure (HTTP, DNS, timeout, etc.)
    #[error("network error: {0}")]
    Network(String),

    /// Parsing failure (HTML, JSON, selector mismatch, etc.)
    #[error("parse error: {0}")]
    Parse(String),

    /// The requested resource does not exist.
    #[error("not found: {0}")]
    NotFound(String),

    /// Website structure changed or unsupported format.
    #[error("unsupported site structure: {0}")]
    Unsupported(String),

    /// Rate limited by remote server.
    #[error("rate limited")]
    RateLimited,

    /// Authentication required or failed.
    #[error("authentication failed")]
    Authentication,

    /// Generic plugin error for site-specific cases.
    #[error("site error: {0}")]
    Site(String),

    /// Storage error propagated from filesystem operations.
    #[error(transparent)]
    Storage(#[from] std::io::Error),

    /// Catch-all fallback.
    #[error("unknown error: {0}")]
    Other(String),
}

// SDK-wide result alias.
//pub type Result<T> = std::result::Result<T, SdkError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = SdkError::NotFound("chapter".into());
        assert_eq!(err.to_string(), "not found: chapter");
    }

    #[test]
    fn test_io_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::Other, "disk");
        let sdk_err: SdkError = io_err.into();

        match sdk_err {
            SdkError::Storage(_) => {
                assert!(sdk_err.to_string().contains("disk"));
            }
            _ => panic!(
                "Expected Storage variant, but failed or not the expected error type: {}",
                sdk_err
            ),
        }
    }
}
