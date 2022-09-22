use anyhow::{anyhow, Result};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

use crate::message::{Config, Frame, Message, MessageEntry, MessageType};
use crate::protocol::{BinaryInputProtocol, BinaryOutputProtocol, TInputProtocol, TOutputProtocol};

pub struct FramedCodec<C>(C);

impl<C> FramedCodec<C> {
    #[allow(dead_code)]
    pub fn new(c: C) -> Self {
        FramedCodec(c)
    }
}

impl<C, T> Encoder<T> for FramedCodec<C>
where
    C: Encoder<T>,
    anyhow::Error: From<C::Error>,
{
    type Error = anyhow::Error;

    fn encode(&mut self, item: T, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let zero_index = dst.len();
        dst.reserve(8);
        unsafe {
            dst.advance_mut(8);
        }
        // Call inner encoder
        self.0.encode(item, dst)?;
        let written = dst.len() - 8 - zero_index;
        let mut buf = &mut dst[zero_index..zero_index + 8];
        buf.put_u64(written as u64);
        Ok(())
    }
}

impl<C> Decoder for FramedCodec<C>
where
    C: Decoder,
    anyhow::Error: From<C::Error>,
{
    type Item = C::Item;
    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 8 {
            // Not enough data to read length marker.
            return Ok(None);
        }

        // Read length marker.
        let mut length_bytes = [0u8; 8];
        length_bytes.copy_from_slice(&src[..8]);
        let length = u64::from_be_bytes(length_bytes) as usize;

        if src.len() < 8 + length {
            src.reserve(8 + length - src.len());

            return Ok(None);
        }

        // Skip the 8-byte length.
        src.advance(8);
        let decoded = self.0.decode(src)?;
        match decoded {
            None => Err(anyhow!(
                "unable to decode message which the data size is enough for decoding"
            )),
            Some(inner) => Ok(Some(inner)),
        }
    }
}

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
