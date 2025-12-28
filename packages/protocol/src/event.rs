use napi_derive::napi;
use puniyu_types::event::Event as puniyu_event;
use puniyu_protocol::event as puniyu_protocol_event;

pub mod message;

#[napi]
pub enum EventReceive {
    Message(message::MessageReceiveEvent),
}

#[napi]
pub enum EventSend {
    Message(message::MessageSendEvent),
}
impl From<puniyu_event> for EventReceive {
    fn from(event: puniyu_event) -> Self {
        match event {
            puniyu_event::Message(message) => {
                let message = *message;
                EventReceive::Message(message.into())
            }
            _ => {
                panic!("Unsupported event type")
            }
        }
    }
}

impl From<EventReceive> for puniyu_event {
    fn from(event: EventReceive) -> Self {
        match event {
            EventReceive::Message(message) => puniyu_event::Message(Box::new(message.into())),
        }
    }
}

impl From<puniyu_protocol_event::event_send::Event> for EventSend {
    fn from(event: puniyu_protocol_event::event_send::Event) -> Self {
        match event {
            puniyu_protocol_event::event_send::Event::MessageEvent(message) => {
                EventSend::Message(message.into())
            }
        }
    }
}

impl From<EventSend> for puniyu_protocol_event::event_send::Event {
    fn from(event: EventSend) -> Self {
        match event {
            EventSend::Message(message) => {
                puniyu_protocol_event::event_send::Event::MessageEvent(message.into())
            }
        }
    }
}

impl From<EventSend> for puniyu_protocol_event::EventSend {
    fn from(event: EventSend) -> Self {
        Self {
            event: Some(event.into()),
        }
    }
}
impl From<puniyu_protocol_event::EventSend> for EventSend {
    fn from(event: puniyu_protocol_event::EventSend) -> Self {
        event.event.unwrap().into()
    }
}