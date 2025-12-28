use super::Scene;
use crate::impl_from_trait;
use napi_derive::napi;
use puniyu_protocol::contact::Contact;
use puniyu_types::contact::FriendContact as puniyu_contact;
use puniyu_protocol::contact::SceneType;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[napi(object)]
pub struct FriendContact {
    /// 事件来源
    pub scene: Scene,
    /// 好友ID
    pub peer: String,
    /// 好友名称
    pub name: Option<String>,
}

impl_from_trait!(FriendContact, puniyu_contact, {
    scene => scene,
    peer => peer,
    name => name,
});

impl From<FriendContact> for Contact {
	fn from(contact: FriendContact) -> Self {
		let scene = SceneType::from(contact.scene);
		Self {
			scene: scene.into(),
			peer: contact.peer,
			name: contact.name,
		}
	}
}

impl From<Contact> for FriendContact {
	fn from(contact: Contact) -> Self {
		let scene = SceneType::try_from(contact.scene).unwrap();
		Self {
			scene: scene.into(),
			peer: contact.peer,
			name: contact.name,
		}
	}
}