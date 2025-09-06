use fume_backend::Backend;

use crate::Steam;

/// A simple trait used for authentication
/// the only reason this trait exists is to simplify Steam's impl
/// SteamApiKey/Unauthorize is the type state of Steam HTTP client
/// different methods are available for different type state
/// same method may return different result for different type state
pub trait Auth: Sized {
    /// returns the query parameters for steam api key
    fn auth(&self) -> Option<(&str, String)>;
    /// provided method for create steam http client from Auth types
    /// ```rust,no_run
    /// use fume::{Auth, SteamApiKey};
    ///
    /// let key = SteamApiKey::new("STEAM_DUMMY_KEY");
    /// let client = reqwest::Client::new();
    /// let steam = key.with_client(client);
    /// ```
    fn with_client<B: Backend>(self, client: B) -> Steam<Self, B> {
        Steam::with_auth_and_client(self, client)
    }
}

/// get your steam web api key from: <https://steamcommunity.com/dev/apikey>
#[derive(Clone, Debug)]
pub struct SteamApiKey {
    pub key: String,
}

impl SteamApiKey {
    /// construct a new steam api key
    pub fn new(key: impl AsRef<str>) -> Self {
        let key = key.as_ref().to_string();
        Self { key }
    }
}

impl Auth for SteamApiKey {
    fn auth(&self) -> Option<(&str, String)> {
        Some(("key", self.key.to_string()))
    }
}

#[derive(Clone, Debug, Default)]
pub struct Unauthorize;

impl Auth for Unauthorize {
    fn auth(&self) -> Option<(&str, String)> {
        None
    }
}
