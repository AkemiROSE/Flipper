use std::sync::Arc;
use tokio::{
    sync::RwLock,
    io::AsyncWriteExt,
    net::TcpStream
};


use anyhow::Result;
use bytes::BytesMut;

use crate::screen::ScreenCap;
use crate::message::{MessageEntry, MessageType, Config, Frame, Message};
use crate::protocol::BinaryOutputProtocol;
use crate::transport::Transport;
use crate::utils::{compress, decompress};

pub struct Service {
    trp: Transport<TcpStream>,
    cap: ScreenCap, 
   
}

impl Service {
    pub fn new(tcp_stream: TcpStream) -> Result<Self> {
        Ok(Self { 
            trp: Transport::new(tcp_stream),
            cap: ScreenCap::new()?,
            
         })
    }

    pub async fn video_service_start(&mut self) -> Result<()> {
        let window_size = self.cap.window_size();
        let config = Config::new(window_size.0 as _, window_size.1 as _);
        let mut last_frame: Vec<u8> = vec![0; config.screen_size as usize];
        self.trp.send(MessageEntry::Config(config)).await?;
 
        loop {
            let mut new_frame = self.cap.capture().await?;
            if last_frame.eq(&new_frame) {continue;}
            last_frame.iter_mut()
                .zip(new_frame.iter())
                .for_each(|(b1, b2)|{
                    *b1 ^= *b2;
                });
            let mut compressed_frame_bytes = compress(last_frame.clone())?;
            let frame = Frame(compressed_frame_bytes);
            self.trp.send(MessageEntry::Frame(frame)).await?; 
            last_frame = new_frame;
        } 
        Ok(())
    }

    
}

