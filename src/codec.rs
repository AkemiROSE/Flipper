use anyhow::{Result, anyhow};
use bytes::{BytesMut, Bytes};
use tokio_util::codec::{Decoder, Encoder};

use crate::message::{Message, MessageEntry, MessageType, Frame, Config};
use crate::protocol::{TOutputProtocol, BinaryOutputProtocol, TInputProtocol, BinaryInputProtocol, self};

pub struct MessageCodec;

impl MessageCodec {
    fn encode<T: Message> (msg: T) -> Vec<u8> {
        let mut buf = BytesMut::new();
        let mut protocol = BinaryOutputProtocol::new(&mut buf);
        msg.encode(&mut protocol);
        buf.to_vec()
    }

    fn decode(bytes: Vec<u8>) -> Result<MessageEntry> {
        let mut buf = Bytes::from(bytes);
        let mut protocol = BinaryInputProtocol::new(buf);
        let message_type =protocol.read_byte()?;
        match MessageType::try_from(message_type) {
            Ok(MessageType::Config) => Ok(MessageEntry::Config(Config::decode(&mut protocol)?)),
            Ok(MessageType::Frame) => Ok(MessageEntry::Frame(Frame::decode(&mut protocol)?)),
            _ => Err(anyhow!("Wrong message type"))
        } 
    }
}