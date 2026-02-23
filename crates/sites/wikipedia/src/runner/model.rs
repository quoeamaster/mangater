use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct WikipediaConfig {
    #[serde(default)]
    pub need_content: bool,
}
