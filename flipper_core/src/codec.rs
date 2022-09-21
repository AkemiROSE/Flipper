use anyhow::{Result, anyhow};
use bytes::{BytesMut, Bytes, Buf};
use tokio_util::codec::{Decoder, Encoder};

use crate::message::{Message, MessageEntry, MessageType, Frame, Config};
use crate::protocol::{TOutputProtocol, BinaryOutputProtocol, TInputProtocol, BinaryInputProtocol};

#[derive(Default)]
pub struct MessageCodec;

impl Encoder<MessageEntry> for MessageCodec {
    type Error = anyhow::Error;
    
    fn encode(&mut self, item: MessageEntry, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let mut protocol = BinaryOutputProtocol::new(dst);
        item.encode(&mut protocol)
    }
}

impl Decoder for MessageCodec {
    type Item = MessageEntry;
    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut protocol = BinaryInputProtocol::new(src);
        let entry = Self::Item::decode(&mut protocol)?;
        Ok(Some(entry))
    }
}
