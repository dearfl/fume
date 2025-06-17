pub trait Backend {
    type Error: std::error::Error;
    fn get(&self, url: &str, query: &[(&str, &str)]) -> Result<String, Self::Error>;
}

#[cfg(feature = "reqwest")]
impl Backend for reqwest::blocking::Client {
    type Error = reqwest::Error;

    fn get(&self, url: &str, query: &[(&str, &str)]) -> Result<String, Self::Error> {
        self.get(url).query(query).send()?.text()
    }
}
