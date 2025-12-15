use super::Scene;
use serde::{Deserialize, Serialize};
use napi_derive::napi;

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