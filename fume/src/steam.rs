use fume_backend::Backend;
use fume_core::{
    Api,
    util::{GetServerInfo, GetServerInfoResponse, GetSupportedApiList, Interface},
};

use crate::{Auth, error::Error};

pub(crate) const HOST: &str = "api.steampowered.com";

#[derive(Clone, Debug)]
pub struct Steam<A: Auth + Copy, B: Backend> {
    pub(crate) auth: A,
    pub(crate) client: B,
    pub(crate) host: &'static str,
}

impl<A: Auth + Copy, B: Backend> Steam<A, B> {
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

    pub fn with_custom_host(self, host: &'static str) -> Self {
        Self { host, ..self }
    }

    pub(crate) async fn request<T: Api>(&self, api: T) -> Result<T::Response, Error<B>> {
        let url = self.url::<T>();
        let query: Vec<_> = self.auth.auth().chain(api.parameters()).collect();
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
