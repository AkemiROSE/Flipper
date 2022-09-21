use anyhow::{Result, anyhow, Error};

use crate::protocol::{TOutputProtocol, BinaryOutputProtocol, TInputProtocol, BinaryInputProtocol};

pub trait Message: Sized {
    fn encode<T: TOutputProtocol>(&self, protocol: &mut T) -> Result<()>;
    fn decode<T: TInputProtocol>(protocol: &mut T) -> Result<Self>;
}

pub enum MessageEntry {
    Config(Config),
    Frame(Frame),

}

impl Message for MessageEntry {
    fn decode<T: TInputProtocol>(protocol: &mut T) -> Result<Self> {
        let messaage_type = protocol.read_byte()?;
        match MessageType::try_from(messaage_type) {
            Ok(MessageType::Config) => Ok(MessageEntry::Config(Config::decode(protocol)?)),
            Ok(MessageType::Frame) => Ok(MessageEntry::Frame(Frame::decode(protocol)?)),
            _ => Err(anyhow!("Wrong message type"))
        } 
    }

    fn encode<T: TOutputProtocol>(&self, protocol: &mut T) -> Result<()> {
        match self {
            MessageEntry::Config(config) => config.encode(protocol),
            MessageEntry::Frame(frame) => frame.encode(protocol),
            _ => Err(anyhow!("wrong message type"))
        }
    }
}

pub struct Config{
    pub width: u64,
    pub height: u64,
    pub frame_size: u64
}

impl Config {
    pub fn new(width: u64, height: u64, frame_size: u64) ->Self {
        Self {
            width,
            height,
            frame_size
        }
    }
}

impl Message for Config {
    fn encode<T: TOutputProtocol>(&self, protocol: &mut T) -> Result<()> {
        protocol.write_byte(u8::from(MessageType::Config))?;
        protocol.write_u64(self.width)?;
        protocol.write_u64(self.height)?;       
        protocol.write_u64(self.frame_size)?; 

        Ok(())
    }

    fn decode<T: TInputProtocol>(protocol: &mut T) -> Result<Self> {
        Ok(Self {
            width: protocol.read_u64()?,
            height: protocol.read_u64()?,
            frame_size: protocol.read_u64()?,
        })
    }

}


pub struct Frame(pub Vec<u8>);

impl Message for Frame {
    fn encode<T: TOutputProtocol>(&self, protocol: &mut T) -> Result<()> {
        protocol.write_byte(u8::from(MessageType::Frame))?;
        protocol.write_bytes(&self.0[..])?;

        Ok(())
    }

    fn decode<T: TInputProtocol>(protocol: &mut T) -> Result<Self> {
        Ok(Self(protocol.read_bytes()?))
    }
   
}

pub enum MessageType {
    Config,
    Frame,
    Audio,
}

impl From<MessageType> for u8 {
    fn from(message_type: MessageType) -> Self {
        match message_type {
            MessageType::Config => 0x00,
            MessageType::Frame => 0x01,
            MessageType::Audio => 0x02,
        }
    }
}

impl TryFrom<u8> for MessageType {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(MessageType::Config),
            0x01 => Ok(MessageType::Frame),
            0x02 => Ok(MessageType::Audio),
            unkn => Err(anyhow!("Unknow message type")),
        }
    }
}
