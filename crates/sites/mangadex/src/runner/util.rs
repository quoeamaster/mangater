use crate::runner::model::AtHomeResponse;
use mangater_sdk::SdkError;

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

pub fn extract_chapter_id_from_url(url: &str) -> Option<String> {
    url.split("/chapter/")
        .nth(1)?
        .split('/')
        .next()
        .map(|s| s.to_string())
}

pub async fn fetch_image_urls(chapter_id: &str) -> Result<Vec<String>, SdkError> {
    // pretend to be curl to avoid being blocked by the server
    let client = reqwest::Client::builder()
        .user_agent("curl/7.88.1")
        .http1_only() // 🔥 critical for MangaDex
        .build()
        .map_err(|e| SdkError::Network(e.to_string()))?;

    let url = format!("https://api.mangadex.org/at-home/server/{}", chapter_id);
    tracing::debug!("api url: {}", url);

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| SdkError::Network(e.to_string()))?;

    let text = resp
        .text()
        .await
        .map_err(|e| SdkError::Network(e.to_string()))?;
    tracing::debug!("{} -> api result content: {:?}", url, text);

    let parsed: AtHomeResponse = serde_json::from_str(&text)
        .map_err(|e: serde_json::Error| SdkError::Parse(e.to_string()))?;

    let urls = parsed
        .chapter
        .data
        .iter()
        .map(|filename| {
            format!(
                "{}/data/{}/{}",
                parsed.base_url, parsed.chapter.hash, filename
            )
        })
        .collect();

    tracing::debug!("{} -> final image urls: {:?}", url, urls);
    Ok(urls)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_subscriber::EnvFilter;

    fn init_tracing() {
        let filter = EnvFilter::new("info");

        let _ = tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_target(false)
            .with_file(true)
            .with_line_number(true)
            .try_init();
    }

    #[test]
    fn test_extract_chapter_id_from_url() {
        assert_eq!(
            extract_chapter_id_from_url("https://mangadex.org/chapter/1234567890"),
            Some("1234567890".to_string())
        );
    }
    #[tokio::test]
    async fn test_fetch_image_urls() {
        init_tracing();

        let urls = fetch_image_urls("5dec8fbf-243e-4c49-9213-11771294792b")
            .await
            .unwrap();
        assert!(!urls.is_empty());
        assert!(urls.len() > 0);

        tracing::debug!("urls: {:?}", urls);
    }
}
