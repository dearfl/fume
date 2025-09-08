use serde::{Deserialize, Serialize};

use crate::{Api, Param, quoted_number};

pub(crate) const INTERFACE: &str = "ISteamUser";
pub(crate) const STEAM_ID_DELTA: u64 = 76561197960265728;

quoted_number!(SteamId);

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

#[derive(Clone, Debug)]
pub struct GetFriendList {
    pub steamid: SteamId,
    pub relationship: Option<Relationship>,
}

impl GetFriendList {
    pub const METHOD: &str = "GetFriendList";
    pub const VERSION: &str = "v1";
}

impl Api for GetFriendList {
    fn interface() -> &'static str {
        INTERFACE
    }

    fn method() -> &'static str {
        Self::METHOD
    }

    fn version() -> &'static str {
        Self::VERSION
    }

    type Response = GetFriendListResponse;

    fn parameters(&self) -> impl Iterator<Item = (&str, String)> {
        std::iter::once((SteamId::name(), self.steamid.value())).chain(
            self.relationship
                .iter()
                .map(|relationship| (Relationship::name(), relationship.value())),
        )
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetFriendListResponse {
    pub friendslist: FriendList,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct FriendList {
    pub friends: Vec<Friend>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Friend {
    pub steamid: SteamId,
    pub relationship: Relationship,
    pub friend_since: u64,
}

#[derive(Clone, Debug)]
pub struct GetUserGroupList {
    pub steamid: SteamId,
}

impl GetUserGroupList {
    pub const METHOD: &str = "GetUserGroupList";
    pub const VERSION: &str = "v1";
}

impl Api for GetUserGroupList {
    fn interface() -> &'static str {
        INTERFACE
    }

    fn method() -> &'static str {
        Self::METHOD
    }

    fn version() -> &'static str {
        Self::VERSION
    }

    type Response = GetUserGroupListResponse;

    fn parameters(&self) -> impl Iterator<Item = (&str, String)> {
        std::iter::once((SteamId::name(), self.steamid.value()))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetUserGroupListResponse {
    pub response: GetUserGroupListResponseWrapper,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetUserGroupListResponseWrapper {
    pub success: bool,
    pub groups: Vec<UserGroup>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct UserGroup {
    pub gid: GroupId,
}

quoted_number!(GroupId);
