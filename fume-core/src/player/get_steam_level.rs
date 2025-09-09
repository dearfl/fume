use serde::{Deserialize, Serialize};

use crate::{Api, Param, Response, user::SteamId};

use super::INTERFACE;

#[derive(Clone, Debug)]
pub struct GetSteamLevel {
    pub steamid: SteamId,
}

impl GetSteamLevel {
    pub const METHOD: &str = "GetSteamLevel";
    pub const VERSION: &str = "v1";
}

impl Api for GetSteamLevel {
    fn interface() -> &'static str {
        INTERFACE
    }

    fn method() -> &'static str {
        Self::METHOD
    }

    fn version() -> &'static str {
        Self::VERSION
    }

    type Response = Response<SteamLevel>;

    fn parameters(&self) -> impl Iterator<Item = (&str, String)> {
        std::iter::once(self.steamid.param())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SteamLevel {
    pub player_level: u64,
}
