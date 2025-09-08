use serde::{Deserialize, Serialize};

use crate::Api;

use super::INTERFACE;

#[derive(Clone, Debug)]
pub struct GetServerInfo;

impl GetServerInfo {
    pub const METHOD: &str = "GetServerInfo";
    pub const VERSION: &str = "v1";
}

impl Api for GetServerInfo {
    fn interface() -> &'static str {
        INTERFACE
    }

    fn method() -> &'static str {
        Self::METHOD
    }

    fn version() -> &'static str {
        Self::VERSION
    }

    type Response = GetServerInfoResponse;

    fn parameters(&self) -> impl Iterator<Item = (&str, String)> {
        std::iter::empty()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetServerInfoResponse {
    pub servertime: u64,
    pub servertimestring: String,
}
