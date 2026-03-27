use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NasaApiResponse {
    #[serde(rename = "collection")]
    pub collection: NasaCollection,
}

#[derive(Debug, Deserialize)]
pub struct NasaCollection {
    pub items: Vec<NasaItem>,
}

#[derive(Debug, Deserialize)]
pub struct NasaItem {
    pub links: Vec<NasaLink>,
}

#[derive(Debug, Deserialize)]
pub struct NasaLink {
    pub href: String,
}
