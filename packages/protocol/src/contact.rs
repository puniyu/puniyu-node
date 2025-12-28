use serde::{Deserialize, Serialize};
use napi_derive::napi;
use puniyu_types::contact::Scene as puniyu_scene;
use puniyu_protocol::contact::SceneType as puniyu_scene_type;
use crate::impl_enum_from_trait;

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
impl_enum_from_trait!(Scene, puniyu_scene, {
	Friend => Friend,
	Group => Group,
});
impl_enum_from_trait!(Scene, puniyu_scene_type, {
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