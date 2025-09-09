use std::time::{Duration, SystemTime, UNIX_EPOCH};

use fume_backend::Backend;
use fume_core::{
    Api,
    app::get_app_list::{App, GetAppList},
    user::{
        SteamId,
        resolve_vanity_url::{ResolveVanityUrl, UrlType, VanityUrl},
    },
    util::{
        get_server_info::{GetServerInfo, GetServerInfoResponse},
        get_supported_api_list::{GetSupportedApiList, Interface},
    },
};

use crate::{
    User,
    auth::{Auth, SteamApiKey},
    error::Error,
    user::Users,
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
    /// ```rust,no_run
    /// use fume::{Auth, Unauthorize};
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let steam = Unauthorize.with_client(reqwest::Client::new());
    ///     let apps = steam.apis().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn apis(&self) -> Result<Vec<Interface>, Error<B>> {
        let api = GetSupportedApiList;
        self.get(api).await.map(|resp| resp.apilist.interfaces)
    }

    /// get server info
    /// ```rust,no_run
    /// use fume::{Auth, Unauthorize};
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let steam = Unauthorize.with_client(reqwest::Client::new());
    ///     let apps = steam.server_info().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn server_info(&self) -> Result<ServerInfo, Error<B>> {
        let api = GetServerInfo;
        self.get(api).await.map(Into::into)
    }

    /// get list of steam apps
    /// ```rust,no_run
    /// use fume::{Auth, Unauthorize};
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let steam = Unauthorize.with_client(reqwest::Client::new());
    ///     let apps = steam.apps().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn apps(&self) -> Result<Vec<App>, Error<B>> {
        let api = GetAppList;
        self.get(api).await.map(|apps| apps.applist.apps)
    }
}

impl<B: Backend> Steam<SteamApiKey, B> {
    pub(crate) fn create_client_ref<D>(&'_ self, value: impl Into<D>) -> SteamRef<'_, B, D> {
        let client = self;
        let value = value.into();
        SteamRef { client, value }
    }

    /// Construct a steam user from 64-bit steam id
    pub fn user(&'_ self, id: impl Into<SteamId>) -> User<'_, B> {
        User(self.create_client_ref(id))
    }

    /// Construct a steam user from custom url
    pub async fn user_from_vanity_url(&'_ self, url: impl AsRef<str>) -> Option<User<'_, B>> {
        self.resolve_vanity_url(url, None)
            .await
            .ok()
            .flatten()
            .map(|value| User(self.create_client_ref(value)))
    }

    /// Construct a steam user from 64-bit steam id
    pub fn users(&'_ self, ids: impl IntoIterator<Item = impl Into<SteamId>>) -> Users<'_, B> {
        let users: Vec<SteamId> = ids.into_iter().map(Into::into).collect();
        Users(self.create_client_ref(users))
    }

    /// resolve user's custom url to 64-bit steam id
    /// ```rust,no_run
    /// use fume::{Auth, SteamApiKey};
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let key = SteamApiKey::new("STEAM_DUMMY_KEY");
    ///     let steam = key.with_client(reqwest::Client::new());
    ///     let id = steam.resolve_vanity_url("dummy", None).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn resolve_vanity_url(
        &self,
        url: impl AsRef<str>,
        url_type: Option<UrlType>,
    ) -> Result<Option<SteamId>, Error<B>> {
        let vanity_url = VanityUrl(url.as_ref().to_string());
        let api = ResolveVanityUrl {
            vanity_url,
            url_type,
        };
        // ignore other fields
        Ok(self.get(api).await?.response.steamid)
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

/// a generic Steam client reference, useful for custom type such as User
#[derive(Clone, Debug)]
pub(crate) struct SteamRef<'s, B: Backend, D = ()> {
    // we could use Arc to remove lifetime annotation, but do we really want to?
    pub(crate) client: &'s Steam<SteamApiKey, B>,
    pub(crate) value: D,
}
