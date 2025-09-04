use fume_backend::Backend;
use fume_core::{
    Api,
    util::{GetServerInfo, GetServerInfoResponse, GetSupportedApiList, Interface},
};

use crate::{SteamApiKey, Unauthorize, error::Error};

pub const HOST: &str = "api.steampowered.com";

#[derive(Clone, Debug)]
pub struct Steam<Auth: Copy, B: Backend + 'static> {
    pub(crate) key: Auth,
    pub(crate) client: B,
    pub(crate) host: &'static str,
}

impl<Auth: Copy, B: Backend + 'static> Steam<Auth, B> {
    pub(crate) fn with_auth_and_client(key: Auth, client: B) -> Self {
        Self {
            key,
            client,
            host: HOST,
        }
    }

    pub fn url<A: Api>(&self) -> String {
        format!(
            "https://{}/{}/{}/{}",
            self.host,
            A::interface(),
            A::method(),
            A::version()
        )
    }
}

impl<B: Backend + 'static> Steam<Unauthorize, B> {
    pub async fn request<A: Api>(&self, api: A) -> Result<A::Response, Error<B>> {
        let url = self.url::<A>();
        let query: Vec<_> = api.parameters().collect();
        let content = self
            .client
            .get(&url, &query)
            .await
            .map_err(|e| Error::BackendError(e))?;
        Ok(serde_json::from_str(&content)?)
    }

    pub async fn apis(&self) -> Result<Vec<Interface>, Error<B>> {
        let api = GetSupportedApiList;
        self.request(api).await.map(|resp| resp.apilist.interfaces)
    }

    pub async fn server_info(&self) -> Result<GetServerInfoResponse, Error<B>> {
        let api = GetServerInfo;
        self.request(api).await
    }
}

impl<B: Backend + 'static> Steam<SteamApiKey, B> {
    pub async fn request<A: Api>(&self, api: A) -> Result<A::Response, Error<B>> {
        let url = self.url::<A>();
        let query: Vec<_> = std::iter::once(("key", self.key.key))
            .chain(api.parameters())
            .collect();
        let content = self
            .client
            .get(&url, &query)
            .await
            .map_err(|e| Error::BackendError(e))?;
        Ok(serde_json::from_str(&content)?)
    }

    pub async fn apis(&self) -> Result<Vec<Interface>, Error<B>> {
        let api = GetSupportedApiList;
        self.request(api).await.map(|resp| resp.apilist.interfaces)
    }

    pub async fn server_info(&self) -> Result<GetServerInfoResponse, Error<B>> {
        let api = GetServerInfo;
        self.request(api).await
    }
}
