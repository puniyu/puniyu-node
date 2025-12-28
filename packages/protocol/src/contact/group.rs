use super::Scene;
use serde::{Deserialize, Serialize};
use napi_derive::napi;
use puniyu_protocol::contact::{Contact, SceneType};
use puniyu_types::contact::GroupContact as puniyu_contact;
use crate::impl_from_trait;

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

impl_from_trait!(GroupContact, puniyu_contact, {
    scene => scene,
	peer => peer,
	name => name,
});

impl From<GroupContact> for Contact {
	fn from(contact: GroupContact) -> Self {
		let scene = SceneType::from(contact.scene);
		Self {
			scene: scene.into(),
			peer: contact.peer,
			name: contact.name,
		}
	}
}

impl From<Contact> for GroupContact {
	fn from(contact: Contact) -> Self {
		let scene = SceneType::try_from(contact.scene).unwrap();
		Self {
			scene: scene.into(),
			peer: contact.peer,
			name: contact.name,
		}
	}
}