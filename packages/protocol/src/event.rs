use napi_derive::napi;
use puniyu_protocol::event as puniyu_event;

pub mod message;

#[napi]
pub enum EventReceive {
    Message(message::MessageReceiveEvent),
}

#[napi]
pub enum EventSend {
    Message(message::MessageSendEvent),
}

impl From<puniyu_event::event_send::Event> for EventSend {
    fn from(event: puniyu_event::event_send::Event) -> Self {
        match event {
            puniyu_event::event_send::Event::MessageEvent(message) => {
                EventSend::Message(message.into())
            }
        }
    }
}

impl From<EventSend> for puniyu_event::event_send::Event {
    fn from(event: EventSend) -> Self {
        match event {
            EventSend::Message(message) => {
                puniyu_event::event_send::Event::MessageEvent(message.into())
            }
        }
    }
}

impl From<EventSend> for puniyu_event::EventSend {
    fn from(event: EventSend) -> Self {
        Self {
            event: Some(event.into()),
        }
    }
}
impl From<puniyu_event::EventSend> for EventSend {
    fn from(event: puniyu_event::EventSend) -> Self {
        event.event.expect("EventSend: event is None").into()
    }
}

impl From<puniyu_event::event_receive::Event> for EventReceive {
    fn from(event: puniyu_event::event_receive::Event) -> Self {
        match event {
            puniyu_event::event_receive::Event::MessageEvent(message) => {
                EventReceive::Message(message.into())
            }
        }
    }
}

impl From<EventReceive> for puniyu_event::event_receive::Event {
    fn from(event: EventReceive) -> Self {
        match event {
            EventReceive::Message(message) => {
                puniyu_event::event_receive::Event::MessageEvent(message.into())
            }
        }
    }
}

impl From<EventReceive> for puniyu_event::EventReceive {
    fn from(event: EventReceive) -> Self {
        Self {
            event: Some(event.into()),
        }
    }
}
impl From<puniyu_event::EventReceive> for EventReceive {
    fn from(event: puniyu_event::EventReceive) -> Self {
        event.event.expect("EventReceive: event is None").into()
    }
}