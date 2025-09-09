use std::time::{Duration, SystemTime, UNIX_EPOCH};

use fume_backend::Backend;
use fume_core::user::{
    GroupId, Relationship, SteamId, get_friend_list::GetFriendList,
    get_user_group_list::GetUserGroupList,
};

use crate::{error::Error, steam::SteamRef};

/// Represent a steam user friend
#[derive(Clone, Debug)]
pub struct Friend {
    /// 64-bit steam user id
    pub id: SteamId,
    /// relationship: all or friend
    pub relationship: Relationship,
    /// became friend since when
    pub since: SystemTime,
}

impl From<&fume_core::user::get_friend_list::Friend> for Friend {
    fn from(value: &fume_core::user::get_friend_list::Friend) -> Self {
        Self {
            id: value.steamid,
            relationship: value.relationship,
            since: UNIX_EPOCH + Duration::from_secs(value.friend_since),
        }
    }
}

/// Represent a steam user
pub struct User<'s, B: Backend>(pub(crate) SteamRef<'s, B, SteamId>);

impl<'s, B: Backend> User<'s, B> {
    /// returns the steamid of user
    pub fn id(&self) -> SteamId {
        self.0.value
    }

    /// request friend list, if a user's friend list is marked as private,
    /// then this will return an HTTP 401 Unauthorized error.
    /// ```rust,no_run
    /// use fume::{Auth, SteamApiKey};
    /// use fume_core::user::Relationship;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let key = SteamApiKey::new("STEAM_DUMMY_KEY");
    ///     let steam = key.with_client(reqwest::Client::new());
    ///     let user = steam.user(76561198335077947u64);
    ///     let friends = user.friends(Some(Relationship::Friend)).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn friends(
        &self,
        relationship: Option<Relationship>,
    ) -> Result<Vec<Friend>, Error<B>> {
        let req = GetFriendList {
            steamid: self.0.value,
            relationship,
        };
        let resp = self.0.client.get(req).await?;
        Ok(resp.friendslist.friends.iter().map(Into::into).collect())
    }

    /// request user group list
    /// ```rust,no_run
    /// use fume::{Auth, SteamApiKey};
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let key = SteamApiKey::new("STEAM_DUMMY_KEY");
    ///     let steam = key.with_client(reqwest::Client::new());
    ///     let user = steam.user(76561198335077947u64);
    ///     let friends = user.groups().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn groups(&self) -> Result<Vec<GroupId>, Error<B>> {
        let req = GetUserGroupList {
            steamid: self.0.value,
        };
        let resp = self.0.client.get(req).await?;
        Ok(resp.response.groups.iter().map(|group| group.gid).collect())
    }
}
