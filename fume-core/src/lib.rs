/// This crate describes steam web apis including its interface,
/// method, version, parameters and response type and etc
/// we separate the definition from http client impl to attempts
/// multiple http client backend support and async/blocking support
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_repr::{Deserialize_repr, Serialize_repr};

pub mod app;
pub mod player;
pub mod user;
pub mod util;

/// this trait describes a specific steam web api
/// a steam web api endpoint usually defines as follow
/// {HTTP_METHOD} https://{host}/{interface}/{method}/{version}?{queries}
/// the `key` parameter is not treated as part of endpoint itself,
/// but rather an authencation parameter hence not include here
pub trait Api {
    // TODO: HTTP method get/post
    /// steam web api interface such as "ISteamWebAPIUtil"
    fn interface() -> &'static str;
    /// steam web api method such as "GetSteamLevel"
    fn method() -> &'static str;
    /// steam web api version
    fn version() -> &'static str;

    type Response: DeserializeOwned;
    // TODO: maybe return &str && &[]?
    fn parameters(&self) -> impl Iterator<Item = (&str, String)>;
}

pub trait Param {
    /// url query name
    fn name() -> &'static str;
    /// url query value
    fn value(&self) -> String;
    /// url query pair
    fn param(&self) -> (&'static str, String) {
        (Self::name(), self.value())
    }
}

/// A generic response type
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Response<T> {
    pub response: T,
}

/// A generic response status type
#[derive(Clone, Debug, Serialize_repr, Deserialize_repr)]
#[cfg_attr(feature = "deny-unknown-fields", serde(deny_unknown_fields))]
#[repr(u8)]
pub enum ResponseResult {
    Success = 1,
    Failure = 42,
}

// some integer types steam wep api returns may be quoted
#[macro_export]
macro_rules! quoted_number {
    ($name:ident) => {
        #[derive(Copy, Clone, Debug, serde::Serialize)]
        #[serde(transparent)]
        pub struct $name(pub u64);

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct _CustomVisitor;

                impl<'de> serde::de::Visitor<'de> for _CustomVisitor {
                    type Value = $name;

                    fn expecting(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        fmt.write_str("integer or string")
                    }

                    fn visit_u64<E>(self, val: u64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok($name(val))
                    }

                    fn visit_str<E>(self, val: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        val.parse::<u64>()
                            .map_err(|_| E::custom(concat!("failed to parse ", stringify!($name))))
                            .map($name)
                    }
                }

                deserializer.deserialize_any(_CustomVisitor)
            }
        }
    };
}
