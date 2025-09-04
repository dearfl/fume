use serde::de::DeserializeOwned;

pub mod util;

pub trait Api {
    fn interface() -> &'static str;
    fn method() -> &'static str;
    fn version() -> &'static str;

    type Response: DeserializeOwned;
    // TODO: maybe return &str && &[]?
    fn parameters(&self) -> impl Iterator<Item = (&str, &str)>;
}
