// sealed?
#[allow(async_fn_in_trait)]
pub trait Backend {
    type Error: std::error::Error;
    async fn get(&self, url: &str, query: &[(&str, &str)]) -> Result<String, Self::Error>;
}

#[cfg(feature = "reqwest")]
impl Backend for reqwest::Client {
    type Error = reqwest::Error;

    async fn get(&self, url: &str, query: &[(&str, &str)]) -> Result<String, Self::Error> {
        self.get(url).query(query).send().await?.text().await
    }
}
