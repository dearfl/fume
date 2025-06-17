use serde::de::DeserializeOwned;

pub const HOST: &str = "api.steampowered.com";
pub const VERSION: &str = "v1";

pub mod steam_web_api_util;

// TODO: maybe return &str && &[]?
pub trait Query {
    type Response: DeserializeOwned;
    fn url() -> String;
    fn query(&self) -> impl Iterator<Item = (&str, &str)>;
}

pub(crate) fn url(interface: &str, method: &str) -> String {
    format!("https://{HOST}/{interface}/{method}/{VERSION}")
}
