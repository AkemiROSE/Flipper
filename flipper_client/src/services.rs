
use std::io::BufReader;
use std::{net::TcpStream, io::Read};
use std::boxed::Box;
use std::thread;
use std::fs::OpenOptions;
use image::{codecs::png, ColorType, ImageEncoder};
use flate2::Compression;
use flate2::read::ZlibDecoder;
use bytes::Bytes;
use anyhow::Result;
use flume::Sender;
use eframe::egui::{ColorImage, Context};
use crate::protocol::BinaryProtocol;
use crate::message::{Config, Message};
pub struct MirrorService {
    ctx: Option<Box<Context>>,
    socket: TcpStream,
    img_sender: Sender<ColorImage>
}

unsafe impl Send for MirrorService{}
unsafe impl Sync for MirrorService{}



impl MirrorService {
    pub fn new(ctx: Option<Box<Context>>, socket: TcpStream, img_sender: Sender<ColorImage>) -> Self {
        Self {           
            ctx, 
            socket ,
            img_sender
        }
    }

    pub fn set_ctx(&mut self, ctx: Box<Context>) {
        self.ctx = Some(ctx)
    }

    pub fn run(&mut self) -> Result<()>{    
        let mut br = BufReader::new(&mut self.socket);
        let mut message_type = vec![0u8; 1];
        let mut message_len = [0u8;8];
        let size = vec![0u8; 2];
        br.read_exact(&mut message_type);
        let config = Config::from_be_bytes(&mut br)?;
        let mut last_frame = vec![0u8; config.frame_size as usize];
     
        loop {
            
            br.read_exact(&mut message_type);
            br.read_exact(&mut message_len);
            let len = u64::from_be_bytes(message_len);
            let mut img_bytes = vec![0; len as usize];
            br.read_exact(&mut img_bytes);
            
            let decompressed_bytes = decompress(&img_bytes[..])?;
            last_frame.iter_mut()
                .zip(decompressed_bytes.iter())
                .for_each(|(b1, b2)|{
                    *b1 ^= *b2;
                });
            let img_bytes = frame_to_img_bytes(&last_frame[..], config.width as _, config.height as _);
            let screenshout = ColorImage::from_rgba_unmultiplied([config.width as _, config.height as _], &img_bytes[..]);
            if let Some(ctx) = self.ctx.as_ref() {
                self.img_sender.send(screenshout)?;              
            }
        }
        Ok(())
    }
}


pub fn decompress(bytes: &[u8]) -> Result<Vec<u8>> {
    let mut gz = ZlibDecoder::new(bytes);
    let mut buf = vec![];
    gz.read_to_end(&mut buf)?;
    Ok(buf)
}

fn frame_to_img_bytes(frame: &[u8], width: usize, height: usize)-> Vec<u8> {
    let mut bitflipped = Vec::with_capacity(width * height * 4);
        let stride = frame.len() / height;
        for y in 0..height {
            for x in 0..width {
                let i = stride * y + 4 * x;
                bitflipped.extend_from_slice(&[
                    frame[i + 2],
                    frame[i + 1],
                    frame[i],
                    255
                ]);
            }
        }
        bitflipped
}