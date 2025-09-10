/// This is the interface brigde between HTTP request and response
/// TODO: maybe we instead construct request object and decode response object?
pub trait Backend: Clone + Sized + 'static {
    type Error: std::error::Error;
    fn get(
        &self,
        url: &str,
        query: &[(&str, String)],
    ) -> impl std::future::Future<Output = Result<String, Self::Error>> + Send;
    // TODO: add post method
}

#[cfg(feature = "reqwest")]
impl Backend for reqwest::Client {
    type Error = reqwest::Error;

    async fn get(&self, url: &str, query: &[(&str, String)]) -> Result<String, Self::Error> {
        self.get(url)
            .query(query)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
    }
}
