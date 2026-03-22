mod runner;

pub use runner::instance::MangadexInstance;

#[cfg(test)]
mod tests {
    use std::io::Write;

    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    #[allow(dead_code)]
    struct AtHomeResponse {
        result: String,

        #[serde(rename = "baseUrl")]
        base_url: String,

        chapter: ChapterData,
    }

    #[derive(Debug, Deserialize)]
    #[allow(dead_code)]
    struct ChapterData {
        hash: String,
        data: Vec<String>,

        #[serde(rename = "dataSaver")]
        data_saver: Option<Vec<String>>,
    }

    #[ignore]
    #[tokio::test]
    async fn test_mangadex_fetch_chapter_images() {
        // actual src url -> https://mangadex.org/chapter/5dec8fbf-243e-4c49-9213-11771294792b
        // api url -> https://api.mangadex.org/at-home/server/5dec8fbf-243e-4c49-9213-11771294792b
        // image urls -> https://cmdxd98sb0x3yprd.mangadex.network/data/356aeccfd848808bf02f70d876a21761/1-f23e6adca067c608216d34e98513dc9e9b7ac9df556c940c4c859d321c6ef967.jpg
        let chapter_id = "5dec8fbf-243e-4c49-9213-11771294792b";

        let url = format!("https://api.mangadex.org/at-home/server/{}", chapter_id);

        // let client = reqwest::Client::new();
        // let client = reqwest::Client::builder()
        //     .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)")
        //     .build()
        //     .unwrap();

        // [lesson] mangaDex only accepts http 1.1 (not 2.0)
        let client = reqwest::Client::builder()
            .user_agent("curl/7.88.1")
            .http1_only() // 🔥 critical
            .build()
            .unwrap();
        // let resp = client
        //     .get(&url)
        //     .send()
        //     .await
        //     .expect("request failed");
        // println!("resp: {:?}", resp);

        let resp = client
            .get(&url)
            .send()
            .await
            .expect("request failed")
            .json::<AtHomeResponse>()
            .await
            .expect("invalid json");

        // Validate response
        assert!(!resp.base_url.is_empty());
        assert!(!resp.chapter.hash.is_empty());
        assert!(!resp.chapter.data.is_empty());

        // Build full image URLs
        let image_urls: Vec<String> = resp
            .chapter
            .data
            .iter()
            .map(|filename| format!("{}/data/{}/{}", resp.base_url, resp.chapter.hash, filename))
            .collect();

        // Print first few images for debugging
        for url in image_urls.iter().take(3) {
            println!("{}", url);
        }
        // Basic assertion
        assert!(image_urls.len() > 0);

        // Step 3: Download image bytes
        let bytes = client
            .get(&image_urls[0].clone())
            .send()
            .await
            .expect("image request failed")
            .bytes()
            .await
            .expect("failed to read bytes");

        assert!(!bytes.is_empty());
        //println!("bytes: {:?}", bytes);

        // Step 4: Save to file
        let file_path = "./testdata/test_image.jpg";
        let mut file = std::fs::File::create(file_path).expect("failed to create file");

        file.write_all(&bytes).expect("failed to write file");
    }
}
