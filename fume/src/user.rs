use std::time::{Duration, SystemTime, UNIX_EPOCH};

use fume_backend::Backend;
use fume_core::user::{GetFriendList, Relationship, SteamId};

use crate::{Steam, SteamApiKey, error::Error};

pub struct User<'s, B: Backend> {
    pub(crate) client: &'s Steam<SteamApiKey, B>,
    pub(crate) id: SteamId,
}

pub struct Friend {
    pub id: SteamId,
    pub relationship: Relationship,
    pub since: SystemTime,
}

impl From<&fume_core::user::Friend> for Friend {
    fn from(value: &fume_core::user::Friend) -> Self {
        Self {
            id: value.steamid,
            relationship: value.relationship,
            since: UNIX_EPOCH + Duration::from_secs(value.friend_since),
        }
    }
}

impl<'s, B: Backend> User<'s, B> {
    pub async fn friends(
        &self,
        relationship: Option<Relationship>,
    ) -> Result<Vec<Friend>, Error<B>> {
        let req = GetFriendList {
            steamid: self.id,
            relationship,
        };
        let resp = self.client.get(req).await?;
        Ok(resp.friendslist.friends.iter().map(Into::into).collect())
    }
}
