use napi_derive::napi;
use puniyu_protocol::event::message as puniyu_message;
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

impl From<MessageSendEvent> for puniyu_message::message_event_send::MessageEvent {
    fn from(event: MessageSendEvent) -> Self {
        match event {
            MessageSendEvent::Friend(message) => {
                puniyu_message::message_event_send::MessageEvent::FriendMessage(message.into())
            }
            MessageSendEvent::Group(message) => {
                puniyu_message::message_event_send::MessageEvent::GroupMessage(message.into())
            }
        }
    }
}

impl From<puniyu_message::message_event_send::MessageEvent> for MessageSendEvent {
    fn from(event: puniyu_message::message_event_send::MessageEvent) -> Self {
        match event {
            puniyu_message::message_event_send::MessageEvent::FriendMessage(message) => {
                MessageSendEvent::Friend(message.into())
            }
            puniyu_message::message_event_send::MessageEvent::GroupMessage(message) => {
                MessageSendEvent::Group(message.into())
            }
        }
    }
}

impl From<MessageSendEvent> for puniyu_message::MessageEventSend {
    fn from(message: MessageSendEvent) -> Self {
        Self {
            message_event: Some(message.into()),
        }
    }
}

impl From<puniyu_message::MessageEventSend> for MessageSendEvent {
    fn from(message: puniyu_message::MessageEventSend) -> Self {
        message.message_event.unwrap().into()
    }
}

impl From<MessageReceiveEvent> for puniyu_message::message_event_receive::MessageEvent {
    fn from(event: MessageReceiveEvent) -> Self {
        match event {
            MessageReceiveEvent::Friend(message) => {
                puniyu_message::message_event_receive::MessageEvent::FriendMessage(message.into())
            }
            MessageReceiveEvent::Group(message) => {
                puniyu_message::message_event_receive::MessageEvent::GroupMessage(message.into())
            }
        }
    }
}

impl From<puniyu_message::message_event_receive::MessageEvent> for MessageReceiveEvent {
    fn from(event: puniyu_message::message_event_receive::MessageEvent) -> Self {
        match event {
            puniyu_message::message_event_receive::MessageEvent::FriendMessage(message) => {
                MessageReceiveEvent::Friend(message.into())
            }
            puniyu_message::message_event_receive::MessageEvent::GroupMessage(message) => {
                MessageReceiveEvent::Group(message.into())
            }
        }
    }
}

impl From<MessageReceiveEvent> for puniyu_message::MessageEventReceive {
    fn from(message: MessageReceiveEvent) -> Self {
        Self {
            message_event: Some(message.into()),
        }
    }
}

impl From<puniyu_message::MessageEventReceive> for MessageReceiveEvent {
    fn from(message: puniyu_message::MessageEventReceive) -> Self {
        message.message_event.unwrap().into()
    }
}
