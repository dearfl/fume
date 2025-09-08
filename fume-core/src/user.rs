use serde::{Deserialize, Serialize};

use crate::{Param, quoted_number};

pub(crate) const INTERFACE: &str = "ISteamUser";
pub(crate) const STEAM_ID_DELTA: u64 = 76561197960265728;

pub mod get_friend_list;
pub mod get_user_group_list;

quoted_number!(SteamId);
quoted_number!(GroupId);

impl From<u64> for SteamId {
    fn from(value: u64) -> Self {
        SteamId(value)
    }
}

impl From<u32> for SteamId {
    fn from(value: u32) -> Self {
        SteamId(u64::from(value) + STEAM_ID_DELTA)
    }
}

impl From<SteamId> for u32 {
    fn from(value: SteamId) -> Self {
        (value.0 - STEAM_ID_DELTA) as u32
    }
}

impl From<SteamId> for u64 {
    fn from(value: SteamId) -> Self {
        value.0
    }
}

impl Param for SteamId {
    fn name() -> &'static str {
        "steamid"
    }

    fn value(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Relationship {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "friend")]
    Friend,
}

impl Param for Relationship {
    fn name() -> &'static str {
        "relationship"
    }

    fn value(&self) -> String {
        match *self {
            Relationship::All => "all".to_string(),
            Relationship::Friend => "friend".to_string(),
        }
    }
}
