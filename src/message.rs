
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
    pub screen_size: u64 
}

impl Config {
    pub fn new(width: u64, height: u64) ->Self {
        Self {
            width,
            height,
            screen_size: width * height * 4 
        }
    }
    pub fn to_be_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.width.to_be_bytes());
        bytes.extend_from_slice(&self.height.to_be_bytes());
        bytes.extend_from_slice(&self.screen_size.to_be_bytes());
        bytes
    }

}


