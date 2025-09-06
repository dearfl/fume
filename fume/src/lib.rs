pub mod error;

mod steam;
pub use steam::*;

mod user;
pub use user::*;

use fume_backend::Backend;

pub trait Auth {
    fn auth(&self) -> Option<(&str, String)>;
}

#[derive(Clone)]
pub struct SteamApiKey {
    pub key: String,
}

impl SteamApiKey {
    pub fn new(key: impl AsRef<str>) -> Self {
        let key = key.as_ref().to_string();
        Self { key }
    }

    pub fn with_client<B: Backend + 'static>(self, client: B) -> Steam<Self, B> {
        Steam::with_auth_and_client(self, client)
    }
}

impl Auth for SteamApiKey {
    fn auth(&self) -> Option<(&str, String)> {
        Some(("key", self.key.to_string()))
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
    fn auth(&self) -> Option<(&str, String)> {
        None
    }
}
