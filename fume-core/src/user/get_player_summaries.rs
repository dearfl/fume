use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    Api, Param, Response,
    app::AppId,
    user::{GroupId, SteamId},
};

use super::INTERFACE;

#[derive(Clone, Debug)]
pub struct SteamIds(pub Vec<SteamId>);

impl Param for SteamIds {
    fn name() -> &'static str {
        "steamids"
    }

    fn value(&self) -> String {
        let values: Vec<String> = self.0.iter().map(|val| val.value()).collect();
        values.join(",")
    }
}

#[derive(Clone, Debug)]
pub struct GetPlayerSummaries {
    pub steamids: SteamIds,
}

impl GetPlayerSummaries {
    pub const METHOD: &str = "GetPlayerSummaries";
    pub const VERSION: &str = "v2";
}

impl Api for GetPlayerSummaries {
    fn interface() -> &'static str {
        INTERFACE
    }

    fn method() -> &'static str {
        Self::METHOD
    }

    fn version() -> &'static str {
        Self::VERSION
    }

    type Response = Response<PlayerSummaries>;

    fn parameters(&self) -> impl Iterator<Item = (&str, String)> {
        std::iter::once(self.steamids.param())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct PlayerSummaries {
    pub players: Vec<PlayerSummary>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct PlayerSummary {
    pub steamid: SteamId,
    #[serde(rename = "communityvisibilitystate")]
    pub community_visibility_state: ProfileVisibility,
    #[serde(rename = "profilestate")]
    pub profile_state: Option<ProfileState>,
    #[serde(rename = "personaname")]
    pub persona_name: String,
    #[serde(rename = "profileurl")]
    pub profile_url: String,
    pub avatar: String,
    #[serde(rename = "avatarmedium")]
    pub avatar_medium: String,
    #[serde(rename = "avatarfull")]
    pub avatar_full: String,
    #[serde(rename = "avatarhash")]
    pub avatar_hash: String,
    #[serde(rename = "lastlogoff")]
    pub last_logoff: Option<u64>,
    #[serde(rename = "personastate")]
    pub persona_state: PersonaState,
    #[serde(rename = "primaryclanid")]
    pub primary_clan_id: Option<GroupId>,
    #[serde(rename = "timecreated")]
    pub time_created: Option<u64>,
    #[serde(rename = "personastateflags")]
    pub persona_state_flags: Option<u8>,
    #[serde(rename = "gameid")]
    pub game_id: Option<AppId>,
    #[serde(rename = "gameserverip")]
    pub game_server_ip: Option<String>,
    #[serde(rename = "gameextrainfo")]
    pub game_extra_info: Option<String>,
    #[serde(rename = "commentpermission")]
    pub comment_permission: Option<u8>,
    #[serde(rename = "realname")]
    pub real_name: Option<String>,
    #[serde(rename = "loccityid")]
    pub loc_city_id: Option<u64>,
    #[serde(rename = "loccountrycode")]
    pub loc_country_code: Option<String>,
    #[serde(rename = "locstatecode")]
    pub loc_state_code: Option<String>,
}

#[derive(Clone, Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum PersonaState {
    Offline = 0,
    Online = 1,
    Busy = 2,
    Away = 3,
    Snooze = 4,
    LookingToTrade = 5,
    LookingToPlay = 6,
}

#[derive(Clone, Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum ProfileVisibility {
    Private = 1,
    FriendsOnly = 2,
    Public = 3,
}

#[derive(Clone, Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum ProfileState {
    Unconfigured = 0,
    Configured = 1,
}
