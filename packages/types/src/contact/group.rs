use super::Scene;
use serde::{Deserialize, Serialize};
use napi_derive::napi;

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