pub trait UrlRewriter: Send + Sync {
    fn rewrite_url(&self, url: &str) -> String;
}
