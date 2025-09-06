use serde::de::DeserializeOwned;

pub mod user;
pub mod util;

pub trait Api {
    fn interface() -> &'static str;
    fn method() -> &'static str;
    fn version() -> &'static str;

    type Response: DeserializeOwned;
    // TODO: maybe return &str && &[]?
    fn parameters(&self) -> impl Iterator<Item = (&str, String)>;
}

pub trait Param {
    fn name() -> &'static str;
    fn value(&self) -> String;
}
