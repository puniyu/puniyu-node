mod contact;

use puniyu_types::contact::ContactType;
use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Message {
// 	/// bot的id
// 	pub bot_id: String,
// 	/// 消息id
// 	pub message_id: String,
// 	/// 消息场景
// 	pub contact: ContactType,
// 	/// 消息发送者
// 	pub sender: SenderType,
// 	/// 消息内容
// 	pub elements: Vec<Elements>,
// }