use std::time::{Duration, SystemTime, UNIX_EPOCH};

use fume_backend::Backend;
use fume_core::{
    Api,
    user::SteamId,
    util::{GetServerInfo, GetServerInfoResponse, GetSupportedApiList, Interface},
};

use crate::{
    User,
    auth::{Auth, SteamApiKey},
    error::Error,
};

pub(crate) const HOST: &str = "api.steampowered.com";

/// Steam HTTP Client
#[derive(Clone, Debug)]
pub struct Steam<A: Auth, B: Backend> {
    pub(crate) auth: A,
    pub(crate) client: B,
    pub(crate) host: &'static str,
}

impl<A: Auth, B: Backend> Steam<A, B> {
    pub(crate) fn with_auth_and_client(key: A, client: B) -> Self {
        Self {
            auth: key,
            client,
            host: HOST,
        }
    }

    pub(crate) fn url<T: Api>(&self) -> String {
        format!(
            "https://{}/{}/{}/{}",
            self.host,
            T::interface(),
            T::method(),
            T::version()
        )
    }

    /// replace the default api host with custom host
    pub fn with_custom_host(self, host: &'static str) -> Self {
        Self { host, ..self }
    }

    pub(crate) async fn get<T: Api>(&self, api: T) -> Result<T::Response, Error<B>> {
        let url = self.url::<T>();
        let query: Vec<_> = self
            .auth
            .auth()
            .into_iter()
            .chain(api.parameters())
            .collect();
        let content = self
            .client
            .get(&url, &query)
            .await
            .map_err(|e| Error::BackendError(e))?;
        Ok(serde_json::from_str(&content)?)
    }

    /// get the availble apis, SteamApiKey and Unauthorize will return different result
    pub async fn apis(&self) -> Result<Vec<Interface>, Error<B>> {
        let api = GetSupportedApiList;
        self.get(api).await.map(|resp| resp.apilist.interfaces)
    }

    /// get server info
    pub async fn server_info(&self) -> Result<ServerInfo, Error<B>> {
        let api = GetServerInfo;
        self.get(api).await.map(Into::into)
    }
}

impl<B: Backend> Steam<SteamApiKey, B> {
    /// Construct a steam user
    pub fn user(&'_ self, id: impl Into<SteamId>) -> User<'_, B> {
        let id = id.into();
        User { client: self, id }
    }
}

#[derive(Clone, Debug)]
pub struct ServerInfo {
    pub servertime: SystemTime,
    pub servertimestring: String,
}

impl From<GetServerInfoResponse> for ServerInfo {
    fn from(value: GetServerInfoResponse) -> Self {
        Self {
            servertime: UNIX_EPOCH + Duration::from_secs(value.servertime),
            servertimestring: value.servertimestring,
        }
    }
}
