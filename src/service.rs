use std::borrow::{Borrow, BorrowMut};
use std::io::Write;
use std::net::TcpStream;
use std::fs::OpenOptions;
use anyhow::Result;
use bytes::BytesMut;
use image::{codecs::png, ColorType, ImageEncoder};
use crate::screen::ScreenCap;
use crate::message::{Message, Config};
use crate::protocol::BinaryProtocol;
use crate::utils::{compress, decompress};

pub struct Service {
    cap: ScreenCap, 
}

impl Service {
    pub fn new() -> Result<Self> {
        Ok(Self { cap: ScreenCap::new()? })
    }

    pub fn video_service_start(&mut self, tcp_stream: &mut TcpStream) -> Result<()> {
        let window_size = self.cap.window_size();
        let config = Config::new(window_size.0 as _, window_size.1 as _);
        let mut config_bytes = config.to_be_bytes();
        config_bytes.insert(0, u8::from(Message::Config));
        tcp_stream.write_all(&mut config_bytes)?;
        
        let mut last_frame: Vec<u8> = vec![0; config.screen_size as usize];
        loop {
            let mut new_frame = self.cap.capture()?;
           /*  let img = OpenOptions::new()
                .write(true)
                .create(true)
                .open(format!("test_{}.png", i)).unwrap();
            let png_file = png::PngEncoder::new(img);
            png_file.write_image(new_frame.as_ref(),  window_size.0 as u32, window_size.1 as u32, ColorType::Rgba8); */
            if last_frame.eq(&new_frame) {continue;}
            last_frame.iter_mut()
                .zip(new_frame.iter())
                .for_each(|(b1, b2)|{
                    *b1 ^= *b2;
                });
            let mut compressed_frame_bytes = compress(last_frame.clone())?;
            let mut buf = BytesMut::new();
            let mut protocol = BinaryProtocol::new(&mut buf);
            protocol.write_byte(u8::from(Message::Frame))?;
            protocol.write_bytes(&compressed_frame_bytes[..])?;
            
            if tcp_stream.write_all(&mut buf).is_err() {
                break;
            }

        }
        Ok(())
    }
}

