use serde::{Deserialize, Serialize};

use crate::{
    Api, Param,
    user::{Relationship, SteamId},
};

use super::INTERFACE;

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
