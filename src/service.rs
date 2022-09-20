use std::sync::Arc;
use tokio::{
    sync::RwLock,
    io::AsyncWriteExt,
    net::TcpStream
};


use anyhow::Result;
use bytes::BytesMut;

use crate::screen::ScreenCap;
use crate::message::{MessageType, Config, Frame, Message};
use crate::protocol::BinaryOutputProtocol;
use crate::transport::Transport;
use crate::utils::{compress, decompress};

pub struct Service {
    cap: ScreenCap, 
   
}

impl Service {
    pub fn new() -> Result<Self> {
        Ok(Self { 
            cap: ScreenCap::new()?,
            
         })
    }

    pub async fn video_service_start(&mut self, transport: Transport) -> Result<()> {
       /*  let window_size = self.cap.window_size();
        let config = Config::new(window_size.0 as _, window_size.1 as _);
        let mut config_bytes = config.to_be_bytes();
        
        self.trp.send_all(&config_bytes[..]).await?;
        
        
        let mut last_frame: Vec<u8> = vec![0; config.screen_size as usize];
        loop {
            let mut new_frame = self.cap.capture().await?;
            if last_frame.eq(&new_frame) {continue;}
            last_frame.iter_mut()
                .zip(new_frame.iter())
                .for_each(|(b1, b2)|{
                    *b1 ^= *b2;
                });
            let mut compressed_frame_bytes = compress(last_frame.clone())?;
            let mut buf = BytesMut::new();
            let mut protocol = BinaryOutputProtocol::new(&mut buf);
            protocol.write_byte(u8::from(MessageType::Frame))?;
            protocol.write_bytes(&compressed_frame_bytes[..])?;
            
            tcp_stream.write_all(&mut buf).await?;
            last_frame = new_frame;
        } */
        Ok(())
    }

    
}

