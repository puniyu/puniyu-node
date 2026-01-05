use crate::bot::BotInfo;
use crate::contact::FriendContact;
use crate::element::receive::Elements;
use crate::sender::FriendSender;
use napi_derive::napi;
use puniyu_protocol::event::message::receive::FriendMessage as puniyu_message;

#[napi(object)]
pub struct FriendMessage {
    pub bot: BotInfo,
    pub event_id: String,
    pub time: u32,
    pub self_id: String,
    pub user_id: String,
    pub message_id: String,
    pub elements: Vec<Elements>,
    pub contact: FriendContact,
    pub sender: FriendSender,
}

impl From<puniyu_message> for FriendMessage {
    fn from(message: puniyu_message) -> Self {
        Self {
            bot: message.friend_message_bot.unwrap_or_default().into(),
            event_id: message.event_id.into(),
            time: message.time as u32,
            self_id: message.self_id,
            user_id: message.user_id,
            message_id: message.message_id,
            elements: message
                .elements
                .into_iter()
                .map(|e| e.into())
                .collect(),
            contact: message.contact.unwrap_or_default().into(),
            sender: message.sender.unwrap_or_default().into(),
        }
    }
}

impl From<FriendMessage> for puniyu_message {
    fn from(message: FriendMessage) -> Self {
        Self {
            friend_message_bot: Some(message.bot.into()),
            event_id: message.event_id ,
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

