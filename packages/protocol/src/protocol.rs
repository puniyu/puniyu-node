use crate::event::{EventReceive, EventSend};
use bytes::Bytes;
use napi::bindgen_prelude::Buffer;
use napi_derive::napi;
use prost::Message;

#[napi(constructor)]
pub struct Protocol;

#[napi]
impl Protocol {
    #[napi]
    pub fn decode(&self, data: Buffer) -> napi::Result<EventReceive> {
        let mut bytes: Bytes = data.to_vec().into();
        let p_type = puniyu_protocol::event::EventReceive::decode(&mut bytes);
        let r_type: puniyu_types::event::Event = p_type.unwrap().into();
        Ok(r_type.into())
    }

    #[napi]
    pub fn encode(&self, data: EventSend) -> napi::Result<Buffer> {
        let p_type: puniyu_protocol::event::EventSend = data.into();
        let bytes = p_type.encode_to_vec();
        Ok(Buffer::from(bytes))
    }
}
