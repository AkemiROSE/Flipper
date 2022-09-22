use std::io::Read;
use anyhow::Result;

pub enum Message {
    Config,
    Frame,
    Audio,
}

impl From<Message> for u8 {
    fn from(message_type: Message) -> Self {
        match message_type {
            Message::Config => 0x00,
            Message::Frame => 0x01,
            Message::Audio => 0x02,
        }
    }
}

impl TryFrom<u8> for Message {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Message::Config),
            0x01 => Ok(Message::Frame),
            0x02 => Ok(Message::Audio),
            unkn => Err(()),
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
            frame_size,
        }
    }
    pub fn to_be_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.width.to_be_bytes());
        bytes.extend_from_slice(&self.height.to_be_bytes());
        bytes.extend_from_slice(&self.frame_size.to_be_bytes());
        bytes
    }
    
    pub fn from_be_bytes<R: Read>(mut buf: R) -> Result<Self> {
        let mut size = [0u8; 8];
        
        buf.read_exact(&mut size);
        let width = u64::from_be_bytes(size);
        buf.read_exact(&mut size);        
        let height = u64::from_be_bytes(size);
        buf.read_exact(&mut size);        
        let frame_size = u64::from_be_bytes(size);


        Ok(Self { 
            width, 
            height, 
            frame_size
        })
    }
}


