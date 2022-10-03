use anyhow::{anyhow, Result};
use bytes::Bytes;
use eframe::egui::{ColorImage, Context};
use flume::Sender;
use std::boxed::Box;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::thread;
use std::time::Duration;
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::runtime::Runtime;

use flipper_core::{
    message::{Config, Frame, Message, MessageEntry, MessageType},
    transport::Transport,
    utils::{decompress, frame_to_img_bytes},
};

pub struct MirrorService {
    
    img_sender: Sender<Box<ColorImage>>,
}

unsafe impl Send for MirrorService {}
unsafe impl Sync for MirrorService {}

impl MirrorService {
    pub fn new( img_sender: Sender<Box<ColorImage>>) -> Self {
        Self { img_sender }
    }

   

    pub fn run(&mut self, ctx: Context) {
        let img_sender = self.img_sender.clone();
        
        let rt = Runtime::new().expect("Unable to create Runtime");
        let _enter = rt.enter();

        std::thread::spawn(move || {
            rt.block_on(async {
                run_video_server("127.0.0.1:8989", ctx, img_sender)
                .await
                .expect("run video server fail")
        
            })
        });
    }
}

pub async fn run_video_server<A: ToSocketAddrs>(
    addr: A,
    ctx: Context,
    img_sender: Sender<Box<ColorImage>>,
) -> Result<()> {
    
    println!("connet to remote");
    let tcp_stream = TcpStream::connect(addr).await?;
    let mut trp = Transport::new(tcp_stream);

    let mut config: Config;
    let mut message = trp.recv_mesage().await.ok_or(anyhow!("failed to recv message"))?;
    match message {
        MessageEntry::Config(conf) => config = conf,
        _ => {
            panic!("first message should be config message.")
        }
    }

    let mut last_frame = vec![0u8; config.frame_size as usize];

    loop {
        match trp.recv_mesage().await {
            Some(message) => match message {
                MessageEntry::Frame(frame) => {
                    let decompressed_bytes = decompress(&frame.0[..])?;
                    last_frame
                        .iter_mut()
                        .zip(decompressed_bytes.iter())
                        .for_each(|(b1, b2)| {
                            *b1 ^= *b2;
                        });
                    let img_bytes =
                        frame_to_img_bytes(&last_frame[..], config.width as _, config.height as _);
                    let screenshout = ColorImage::from_rgba_unmultiplied(
                        [config.width as _, config.height as _],
                        &img_bytes[..],
                    );             
                   
                    img_sender.send(Box::new(screenshout))?;
                    ctx.request_repaint();
                }
                _ => (),
            },
            None => {}
        }
    }
}
