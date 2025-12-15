use serde::{Deserialize, Serialize};
use napi_derive::napi;

mod friend;
use friend::FriendContact;
mod group;
use group::GroupContact;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[napi]
/// 场景
pub enum Scene {
	/// 群聊场景
	Group,
	/// 好友场景
	Friend,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[napi]
pub enum ContactType {
	/// 好友联系人
	Friend(FriendContact),
	/// 群聊联系人
	Group(GroupContact),
}