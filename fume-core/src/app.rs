use serde::{Deserialize, Serialize};

use crate::Api;

pub(crate) const INTERFACE: &str = "ISteamApps";

#[derive(Clone, Debug)]
pub struct GetAppList;

impl GetAppList {
    pub const METHOD: &str = "GetAppList";
    pub const VERSION: &str = "v2";
}

impl Api for GetAppList {
    fn interface() -> &'static str {
        INTERFACE
    }

    fn method() -> &'static str {
        Self::METHOD
    }

    fn version() -> &'static str {
        Self::VERSION
    }

    type Response = GetAppListResponse;

    fn parameters(&self) -> impl Iterator<Item = (&str, String)> {
        std::iter::empty()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetAppListResponse {
    pub applist: AppList,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AppList {
    pub apps: Vec<App>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct App {
    pub appid: AppId,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppId(pub u64);
