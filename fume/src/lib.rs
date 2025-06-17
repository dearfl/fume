pub mod error;

#[cfg(feature = "async")]
mod steam_async;

#[cfg(feature = "async")]
pub use steam_async::*;

#[cfg(feature = "blocking")]
mod steam_blocking;

#[cfg(feature = "blocking")]
pub use steam_blocking::*;

#[cfg(feature = "async")]
use fume_async::Backend;

#[cfg(feature = "blocking")]
use fume_blocking::Backend;

#[derive(Clone, Copy, Debug, Default)]
pub struct SteamApiKey {
    pub key: &'static str,
}

impl SteamApiKey {
    pub fn new(key: impl AsRef<str>) -> Self {
        let key = String::leak(key.as_ref().to_owned());
        Self { key }
    }

    pub fn with_client<B: Backend + 'static>(self, client: B) -> Steam<Self, B> {
        let client = Box::leak(Box::new(client));
        Steam { key: self, client }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Unauthorized;

impl Unauthorized {
    pub fn with_client<B: Backend + 'static>(client: B) -> Steam<Self, B> {
        let client = Box::leak(Box::new(client));
        Steam { key: Self, client }
    }
}
