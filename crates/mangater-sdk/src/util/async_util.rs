use tokio::runtime::{Handle, Runtime};

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
