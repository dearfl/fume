use serde::{Deserialize, Serialize};

use crate::{
    Api, Param, Response,
    user::{GroupId, SteamId},
};

use super::INTERFACE;

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

    type Response = Response<GetUserGroupListResponseInner>;

    fn parameters(&self) -> impl Iterator<Item = (&str, String)> {
        std::iter::once(self.steamid.param())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetUserGroupListResponseInner {
    pub success: bool,
    pub groups: Vec<UserGroup>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct UserGroup {
    pub gid: GroupId,
}
