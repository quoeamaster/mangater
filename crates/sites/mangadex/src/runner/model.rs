use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AtHomeResponse {
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    pub result: String,

    pub chapter: ChapterData,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ChapterData {
    pub hash: String,
    pub data: Vec<String>,

    #[serde(rename = "dataSaver")]
    pub data_saver: Option<Vec<String>>,
}
