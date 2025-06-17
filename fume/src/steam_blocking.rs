use fume_blocking::Backend;
use fume_core::{
    Query,
    steam_web_api_util::{
        GetServerInfo, GetServerInfoResponse, GetSupportedApiList, GetSupportedApiListResponse,
    },
};

use crate::{SteamApiKey, Unauthorized, error::Error};

#[derive(Clone, Copy, Debug)]
pub struct Steam<K: Copy, B: Backend + 'static> {
    pub(crate) key: K,
    pub(crate) client: &'static B,
}

impl<B: Backend + 'static> Steam<Unauthorized, B> {
    pub fn request<Q: Query>(&self, api: Q) -> Result<Q::Response, Error<B>> {
        let url = Q::url();
        let query: Vec<_> = api.query().collect();
        let content = self
            .client
            .get(&url, &query)
            .map_err(|e| Error::BackendError(e))?;
        Ok(serde_json::from_str(&content)?)
    }

    pub fn apis(&self) -> Result<GetSupportedApiListResponse, Error<B>> {
        let api = GetSupportedApiList;
        self.request(api)
    }

    pub fn server_info(&self) -> Result<GetServerInfoResponse, Error<B>> {
        let api = GetServerInfo;
        self.request(api)
    }
}

impl<B: Backend + 'static> Steam<SteamApiKey, B> {
    pub fn request<Q: Query>(&self, api: Q) -> Result<Q::Response, Error<B>> {
        let url = Q::url();
        let query: Vec<_> = std::iter::once(("key", self.key.key))
            .chain(api.query())
            .collect();
        let content = self
            .client
            .get(&url, &query)
            .map_err(|e| Error::BackendError(e))?;
        Ok(serde_json::from_str(&content)?)
    }

    pub fn apis(&self) -> Result<GetSupportedApiListResponse, Error<B>> {
        let api = GetSupportedApiList;
        self.request(api)
    }

    pub fn server_info(&self) -> Result<GetServerInfoResponse, Error<B>> {
        let api = GetServerInfo;
        self.request(api)
    }
}
