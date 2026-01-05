use crate::impl_enum_from_trait;
use napi_derive::napi;
use puniyu_protocol::contact as puniyu_contact;
use puniyu_protocol::contact::SceneType;
use serde::{Deserialize, Serialize};

mod friend;
pub use friend::FriendContact;
mod group;
pub use group::GroupContact;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[napi]
/// 场景
pub enum Scene {
    /// 群聊场景
    Group,
    /// 好友场景
    Friend,
}

impl_enum_from_trait!(Scene, SceneType, {
    Friend => Friend,
    Group => Group,
});

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[napi]
pub enum ContactType {
    /// 好友联系人
    Friend(FriendContact),
    /// 群聊联系人
    Group(GroupContact),
}

impl From<puniyu_contact::Contact> for ContactType {
    fn from(contact: puniyu_contact::Contact) -> Self {
        let scene = SceneType::try_from(contact.scene).unwrap_or_default();
        match scene {
            SceneType::Friend => Self::Friend(FriendContact {
                scene: Scene::Friend,
                peer: contact.peer,
                name: contact.name,
            }),
            SceneType::Group => Self::Group(GroupContact {
                scene: Scene::Group,
                peer: contact.peer,
                name: contact.name,
            }),
        }
    }
}


impl From<ContactType> for puniyu_contact::Contact {
    fn from(contact: ContactType) -> Self {
        match contact {
            ContactType::Friend(friend) => Self {
                scene: SceneType::Friend.into(),
                peer: friend.peer,
                name: friend.name,
            },
            ContactType::Group(group) => Self {
                scene: SceneType::Group.into(),
                peer: group.peer,
                name: group.name,
            }
        }
    }
}