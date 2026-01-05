use super::Scene;
use napi_derive::napi;
use puniyu_protocol::contact as puniyu_contact;
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


impl From<FriendContact> for puniyu_contact::Contact {
	fn from(contact: FriendContact) -> Self {
		let scene = SceneType::from(contact.scene);
		Self {
			scene: scene.into(),
			peer: contact.peer,
			name: contact.name,
		}
	}
}

impl From<puniyu_contact::Contact> for FriendContact {
	fn from(contact: puniyu_contact::Contact) -> Self {
		let scene = SceneType::try_from(contact.scene).unwrap();
		Self {
			scene: scene.into(),
			peer: contact.peer,
			name: contact.name,
		}
	}
}