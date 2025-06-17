use serde::{Deserialize, Serialize};

use crate::{Query, url};

pub const INTERFACE: &str = "ISteamWebAPIUtil";

#[derive(Clone, Debug)]
pub struct GetServerInfo;

impl GetServerInfo {
    pub const METHOD: &str = "GetServerInfo";
}

impl Query for GetServerInfo {
    type Response = GetServerInfoResponse;

    fn url() -> String {
        url(INTERFACE, Self::METHOD)
    }

    fn query(&self) -> impl Iterator<Item = (&str, &str)> {
        std::iter::empty()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetServerInfoResponse {
    pub servertime: u64,
    pub servertimestring: String,
}

#[derive(Clone, Debug)]
pub struct GetSupportedApiList;

impl GetSupportedApiList {
    pub const METHOD: &str = "GetSupportedAPIList";
}

impl Query for GetSupportedApiList {
    type Response = GetSupportedApiListResponse;

    fn url() -> String {
        url(INTERFACE, Self::METHOD)
    }

    fn query(&self) -> impl Iterator<Item = (&str, &str)> {
        std::iter::empty()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
#[serde(rename_all = "lowercase")]
pub enum ParameterType {
    Bool,
    Int8,
    UInt8,
    Int16,
    UInt16,
    Int32,
    Uint32,
    Int64,
    Uint64,
    String,
    #[serde(rename = "{enum}")]
    Enum,
    #[serde(rename = "{message}")]
    Message,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Parameter {
    pub name: String,
    pub r#type: ParameterType,
    pub optional: bool,
    #[serde(default)]
    pub description: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Method {
    pub name: String,
    pub version: i32,
    pub httpmethod: HttpMethod,
    pub parameters: Vec<Parameter>,
    #[serde(default)]
    pub description: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Interface {
    pub name: String,
    pub methods: Vec<Method>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ApiList {
    pub interfaces: Vec<Interface>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetSupportedApiListResponse {
    pub apilist: ApiList,
}
