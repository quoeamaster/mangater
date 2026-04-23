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

//! util/async_util.rs provides utilities for asynchronous operations.

use tokio::runtime::{Handle, Runtime};

/// helper function to block on an asynchronous operation
///
/// # Arguments
/// * `future`: The future to block on.
///
/// # Returns
/// * `T` if the future is completed successfully.
/// * `Err(SdkError)` if the future cannot be blocked on.
pub fn block_on_async<F, T>(future: F) -> T
where
    F: std::future::Future<Output = T>,
{
    // Case 1: already inside Tokio runtime — can't block the current async worker thread.
    // Use block_in_place to move to a blocking thread, then create a nested runtime there.
    if Handle::try_current().is_ok() {
        tokio::task::block_in_place(|| {
            let rt = Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(future)
        })
    } else {
        // Case 2: no runtime → create one
        let rt = Runtime::new().expect("Failed to create Tokio runtime");
        rt.block_on(future)
    }
}
