#[allow(async_fn_in_trait)]
pub trait Backend: Clone + 'static {
    type Error: std::error::Error;
    async fn get(&self, url: &str, query: &[(&str, String)]) -> Result<String, Self::Error>;
}

#[cfg(feature = "reqwest")]
impl Backend for reqwest::Client {
    type Error = reqwest::Error;

    async fn get(&self, url: &str, query: &[(&str, String)]) -> Result<String, Self::Error> {
        self.get(url).query(query).send().await?.text().await
    }
}
