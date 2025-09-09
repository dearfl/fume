use serde::{Deserialize, Serialize};

use crate::{Api, Param, Response, ResponseResult, user::SteamId};

use super::INTERFACE;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VanityUrl(pub String);

impl Param for VanityUrl {
    fn name() -> &'static str {
        "vanityurl"
    }

    fn value(&self) -> String {
        self.0.to_owned()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum UrlType {
    #[default]
    IndividualProfile = 1,
    Group = 2,
    OfficialGameGroup = 3,
}

impl Param for UrlType {
    fn name() -> &'static str {
        "url_type"
    }

    fn value(&self) -> String {
        let num = match *self {
            UrlType::IndividualProfile => 1,
            UrlType::Group => 2,
            UrlType::OfficialGameGroup => 3,
        };
        format!("{}", num)
    }
}

#[derive(Clone, Debug)]
pub struct ResolveVanityUrl {
    pub vanity_url: VanityUrl,
    pub url_type: Option<UrlType>,
}

impl ResolveVanityUrl {
    pub const METHOD: &str = "ResolveVanityURL";
    pub const VERSION: &str = "v1";
}

impl Api for ResolveVanityUrl {
    fn interface() -> &'static str {
        INTERFACE
    }

    fn method() -> &'static str {
        Self::METHOD
    }

    fn version() -> &'static str {
        Self::VERSION
    }

    type Response = Response<ResolveVanityUrlResponseInner>;

    fn parameters(&self) -> impl Iterator<Item = (&str, String)> {
        std::iter::once(self.vanity_url.param())
            .chain(self.url_type.iter().map(|url_type| url_type.param()))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ResolveVanityUrlResponseInner {
    pub success: ResponseResult,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub steamid: Option<SteamId>,
}
