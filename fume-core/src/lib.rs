use serde::{Deserialize, Serialize, de::DeserializeOwned};

pub mod app;
pub mod player;
pub mod user;
pub mod util;

pub trait Api {
    fn interface() -> &'static str;
    fn method() -> &'static str;
    fn version() -> &'static str;

    type Response: DeserializeOwned;
    // TODO: maybe return &str && &[]?
    fn parameters(&self) -> impl Iterator<Item = (&str, String)>;
}

pub trait Param {
    fn name() -> &'static str;
    fn value(&self) -> String;
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

pub(crate) use quoted_number;
use serde_repr::{Deserialize_repr, Serialize_repr};
