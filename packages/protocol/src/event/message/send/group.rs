use crate::bot::BotInfo;
use crate::contact::GroupContact;
use crate::element::send::Elements;
use crate::sender::GroupSender;
use napi_derive::napi;
use puniyu_protocol::event::message::send::GroupMessage as puniyu_protocol_message;

#[napi(object)]
pub struct GroupMessage {
    pub bot: BotInfo,
    pub event_id: String,
    pub time: u32,
    pub self_id: String,
    pub user_id: String,
    pub message_id: String,
    pub elements: Vec<Elements>,
    pub contact: GroupContact,
    pub sender: GroupSender,
}

impl From<GroupMessage> for puniyu_protocol_message {
    fn from(message: GroupMessage) -> Self {
        Self {
            group_message_bot: Some(message.bot.into()),
            event_id: message.event_id,
            time: message.time as u64,
            self_id: message.self_id,
            user_id: message.user_id,
            message_id: message.message_id,
            elements: message.elements.into_iter().map(|e| e.into()).collect(),
            contact: Some(message.contact.into()),
            sender: Some(message.sender.into()),
        }
    }
}

impl From<puniyu_protocol_message> for GroupMessage{
    fn from(message: puniyu_protocol_message) -> Self {
        Self {
            bot: message.group_message_bot.unwrap().into(),
            event_id: message.event_id,
            time: message.time as u32,
            self_id: message.self_id,
            user_id: message.user_id,
            message_id: message.message_id,
            elements: message.elements.into_iter().map(|e| e.into()).collect(),
            contact: message.contact.unwrap().into(),
            sender: message.sender.unwrap().into(),
        }
    }
}