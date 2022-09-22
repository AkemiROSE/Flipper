
use std::io::BufReader;
use std::boxed::Box;
use std::thread;
use std::fs::OpenOptions;
use std::time::Duration;
use bytes::Bytes;
use anyhow::{Result, anyhow};
use flume::Sender;
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::runtime::Runtime;
use eframe::egui::{ColorImage, Context};

use flipper_core::{
    transport::Transport,
    message::{Message, MessageType, MessageEntry, Config, Frame},
    utils::{decompress, frame_to_img_bytes}
};


pub struct MirrorService {
    ctx: Option<Box<Context>>,
    img_sender: Sender<ColorImage>
}

unsafe impl Send for MirrorService{}
unsafe impl Sync for MirrorService{}



impl MirrorService {
    pub fn new(ctx: Option<Box<Context>>, img_sender: Sender<ColorImage>) -> Self {
        Self {           
            ctx, 
           
            img_sender
        }
    }

    pub fn set_ctx(&mut self, ctx: Box<Context>) {
        self.ctx = Some(ctx)
    }

    pub fn run(&mut self){    
        let img_sender = self.img_sender.clone();
        let rt = Runtime::new().expect("Unable to create Runtime");
        let _enter = rt.enter();
        
        tokio::spawn(async {
            run_video_server("127.0.0.1:8989", img_sender).await.expect("run video server fail")
        });

        std::thread::spawn(move || {
            rt.block_on(async {
                loop {
                    tokio::time::sleep(Duration::from_secs(1000)).await;
                }
            })
        });

    }

   
}

pub async fn run_video_server<A: ToSocketAddrs>(addr: A, img_sender: Sender<ColorImage>) -> Result<()> {
    println!("connet to remote");
    let tcp_stream = TcpStream::connect(addr).await?;
    let mut trp = Transport::new(tcp_stream);

    let mut config: Config;
    let mut message =  trp.recv_mesage().await.ok_or(anyhow!(""))?; 
    match message {
        MessageEntry::Config(conf) => {config = conf},
        _ => {panic!("first message should be config message.")}
    }
    //println!("Recved config: width:{}, height:{}, frame_size:{}",config.width, config.height, config.frame_size);
    let mut last_frame = vec![0u8; config.frame_size as usize];
   
    loop {    
        match trp.recv_mesage().await {
            Some(message) => {
                match message{
                    MessageEntry::Frame(frame) => {
                        let decompressed_bytes = decompress(&frame.0[..])?;
                        last_frame.iter_mut()
                            .zip(decompressed_bytes.iter())
                            .for_each(|(b1, b2)|{
                                *b1 ^= *b2;
                            });
                        let img_bytes = frame_to_img_bytes(&last_frame[..], config.width as _, config.height as _);
                        let screenshout = ColorImage::from_rgba_unmultiplied([config.width as _, config.height as _], &img_bytes[..]);
                        img_sender.send(screenshout)?;       
                    }
                    _ =>()
                }
            }
            None =>{}
        }         
        
    }         
}




