use crate::bot::BotInfo;
use crate::contact::FriendContact;
use crate::element::receive::Elements;
use crate::sender::FriendSender;
use napi_derive::napi;
use puniyu_types::event::EventBase;
use puniyu_types::event::message::FriendMessage as puniyu_message;
use puniyu_types::event::message::MessageBase;
use puniyu_types::event::message::MessageBuilder;

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
            bot: message.bot().to_owned().into(),
            event_id: message.event_id().into(),
            time: message.time() as u32,
            self_id: message.self_id().into(),
            user_id: message.user_id().into(),
            message_id: message.message_id().into(),
            elements: message
                .elements()
                .iter()
                .map(|e| e.to_owned().into())
                .collect(),
            contact: message.contact().into(),
            sender: message.sender().into(),
        }
    }
}

impl From<FriendMessage> for puniyu_message {
    fn from(message: FriendMessage) -> Self {
        let builder = MessageBuilder {
            bot: message.bot.into(),
            event_id: message.event_id.into(),
            self_id: message.self_id,
            user_id: message.user_id,
            contact: message.contact.into(),
            sender: message.sender.into(),
            time: message.time as u64,
            message_id: message.message_id,
            elements: message.elements.into_iter().map(|e| e.into()).collect(),
        };
        Self::new(builder)
    }
}

