use crate::event::EventReceive;
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
        let p_type = puniyu_protocol::event::EventReceive::decode(&mut bytes).map_err(|e| napi::Error::from_reason(e.to_string()))?;
        Ok(p_type.into())
    }

    #[napi]
    pub fn encode(&self, data: EventReceive) -> napi::Result<Buffer> {
        let p_type: puniyu_protocol::event::EventReceive = data.into();
        let bytes = p_type.encode_to_vec();
        Ok(Buffer::from(bytes))
    }
}
