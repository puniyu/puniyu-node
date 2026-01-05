use super::Scene;
use serde::{Deserialize, Serialize};
use napi_derive::napi;
use puniyu_protocol::contact as puniyu_contact;
use puniyu_protocol::contact::SceneType;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[napi(object)]
pub struct GroupContact {
	/// 事件来源
	pub scene: Scene,
	/// 群聊id
	pub peer: String,
	/// 群名称
	pub name: Option<String>,
}


impl From<GroupContact> for puniyu_contact::Contact {
	fn from(contact: GroupContact) -> Self {
		let scene = SceneType::from(contact.scene);
		Self {
			scene: scene.into(),
			peer: contact.peer,
			name: contact.name,
		}
	}
}

impl From<puniyu_contact::Contact> for GroupContact {
	fn from(contact: puniyu_contact::Contact) -> Self {
		let scene = SceneType::try_from(contact.scene).unwrap();
		Self {
			scene: scene.into(),
			peer: contact.peer,
			name: contact.name,
		}
	}
}