use napi_derive::napi;
use puniyu_protocol::event::message as puniyu_protocol_message;
use puniyu_types::event::message::MessageEvent as puniyu_message;
pub mod receive;
pub mod send;

#[napi]
pub enum MessageReceiveEvent {
    Friend(receive::friend::FriendMessage),
    Group(receive::group::GroupMessage),
}

#[napi]
pub enum MessageSendEvent {
    Friend(send::friend::FriendMessage),
    Group(send::group::GroupMessage),
}

impl From<MessageReceiveEvent> for puniyu_message {
    fn from(event: MessageReceiveEvent) -> Self {
        match event {
            MessageReceiveEvent::Friend(message) => puniyu_message::Friend(message.into()),
            MessageReceiveEvent::Group(message) => puniyu_message::Group(message.into()),
        }
    }
}

impl From<puniyu_message> for MessageReceiveEvent {
    fn from(event: puniyu_message) -> Self {
        match event {
            puniyu_message::Friend(message) => MessageReceiveEvent::Friend(message.into()),
            puniyu_message::Group(message) => MessageReceiveEvent::Group(message.into()),
        }
    }
}

impl From<MessageSendEvent> for puniyu_protocol_message::message_event_send::MessageEvent {
    fn from(event: MessageSendEvent) -> Self {
        match event {
            MessageSendEvent::Friend(message) => {
                puniyu_protocol_message::message_event_send::MessageEvent::FriendMessage(
                    message.into(),
                )
            }
            MessageSendEvent::Group(message) => {
                puniyu_protocol_message::message_event_send::MessageEvent::GroupMessage(
                    message.into(),
                )
            }
        }
    }
}

impl From<puniyu_protocol_message::message_event_send::MessageEvent> for MessageSendEvent {
    fn from(event: puniyu_protocol_message::message_event_send::MessageEvent) -> Self {
        match event {
            puniyu_protocol_message::message_event_send::MessageEvent::FriendMessage(message) => {
                MessageSendEvent::Friend(message.into())
            }
            puniyu_protocol_message::message_event_send::MessageEvent::GroupMessage(message) => {
                MessageSendEvent::Group(message.into())
            }
        }
    }
}

impl From<MessageSendEvent> for puniyu_protocol_message::MessageEventSend {
    fn from(message: MessageSendEvent) -> Self {
        Self {
            message_event: Some(message.into()),
        }
    }
}

impl From<puniyu_protocol_message::MessageEventSend> for MessageSendEvent {
    fn from(message: puniyu_protocol_message::MessageEventSend) -> Self {
        message.message_event.unwrap().into()
    }
}