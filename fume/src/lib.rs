pub mod error;

mod steam;
pub use steam::*;

use fume_backend::Backend;

pub trait Auth {
    fn auth(&self) -> impl Iterator<Item = (&str, &str)>;
}

#[derive(Clone, Copy)]
pub struct SteamApiKey {
    pub key: &'static str,
}

impl SteamApiKey {
    pub fn new(key: impl AsRef<str>) -> Self {
        let key = String::leak(key.as_ref().to_owned());
        Self { key }
    }

    pub fn with_client<B: Backend + 'static>(self, client: B) -> Steam<Self, B> {
        Steam::with_auth_and_client(self, client)
    }
}

impl Auth for SteamApiKey {
    fn auth(&self) -> impl Iterator<Item = (&str, &str)> {
        std::iter::once(("key", self.key))
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Unauthorize;

impl Unauthorize {
    pub fn with_client<B: Backend + 'static>(client: B) -> Steam<Self, B> {
        Steam::with_auth_and_client(Unauthorize, client)
    }
}

impl Auth for Unauthorize {
    fn auth(&self) -> impl Iterator<Item = (&str, &str)> {
        std::iter::empty()
    }
}
