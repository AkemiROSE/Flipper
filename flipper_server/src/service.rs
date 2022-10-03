use std::sync::Arc;
use tokio::{
    io::AsyncWriteExt, 
    net::TcpStream, 
    sync::mpsc::{unbounded_channel, UnboundedSender, UnboundedReceiver, Receiver},
    sync::RwLock};

use anyhow::Result;
use bytes::BytesMut;

use flipper_core::message::{Config, Frame, Message, MessageEntry, MessageType};
use flipper_core::protocol::BinaryOutputProtocol;
use flipper_core::screen::ScreenCap;
use flipper_core::transport::Transport;
use flipper_core::utils::{compress, decompress};

pub struct Service {
    trp: Transport<TcpStream>,
    cap: ScreenCap,
    message_tx: UnboundedSender<MessageEntry>,
    message_rx: UnboundedReceiver<MessageEntry>,
    shutdown_recver: Receiver<()>,
}

impl Service {
    pub fn new(tcp_stream: TcpStream, shutdown_recver: Receiver<()>) -> Result<Self> {
        let (message_tx, message_rx) = unbounded_channel();
        Ok(Self {
            trp: Transport::new(tcp_stream),
            cap: ScreenCap::new()?,
            message_tx,
            message_rx,
            shutdown_recver,
        })
    }


    pub async fn video_service_start(&mut self) -> Result<()> {
        let window_size = self.cap.window_size();
        let mut last_frame = self.cap.capture().await?;

        let config = Config::new(
            window_size.0 as _,
            window_size.1 as _,
            last_frame.len() as _,
        );
        self.trp.send_mesage(MessageEntry::Config(config)).await?;
        //send first frame
        let mut compressed_frame_bytes = compress(&last_frame[..])?;
        let frame = Frame(compressed_frame_bytes);
        self.trp.send_mesage(MessageEntry::Frame(frame)).await?;

        loop {
            let mut new_frame = self.cap.capture().await?;
            if last_frame.eq(&new_frame) {
                continue;
            }
            last_frame
                .iter_mut()
                .zip(new_frame.iter())
                .for_each(|(b1, b2)| {
                    *b1 ^= *b2;
                });
            let mut compressed_frame_bytes = compress(&last_frame[..])?;
            let frame = Frame(compressed_frame_bytes);
            self.trp.send_mesage(MessageEntry::Frame(frame)).await?;
            last_frame = new_frame;
        }
        Ok(())
    }
}
