pub trait UrlFilter: Send + Sync {
    fn filter_url(&self, url: &str) -> bool;
}
