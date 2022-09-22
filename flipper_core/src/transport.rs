use std::sync::Arc;
use std::marker::Unpin;
use bytes::{BytesMut, Bytes, Buf};
use tokio::{
    net::TcpStream,
    io::{AsyncWriteExt, AsyncReadExt, AsyncRead, AsyncWrite, BufStream}, 
    sync::RwLock
};
use tokio_stream::Stream;
use tokio_util::codec::Framed;
use futures_util::{SinkExt, StreamExt};
use anyhow::{Result, anyhow};

use crate::message::{Message, MessageEntry};
use crate::codec::{FramedCodec,MessageCodec};

pub struct Transport<T: AsyncRead + AsyncWrite + Unpin>(Framed<T, FramedCodec<MessageCodec>>);


impl<T: AsyncRead + AsyncWrite + Unpin> Transport<T> {
    pub fn new(io: T) -> Self {
        let codec = MessageCodec::default();
        Transport(Framed::new(io, FramedCodec::new(codec)))
    } 

    pub async fn send_mesage(&mut self, msg: MessageEntry) -> Result<()> {
        self.0.send(msg).await?;
        Ok(())
    }

    pub async fn recv_mesage(&mut self) -> Option<MessageEntry> {
        return match self.0.next().await {
            Some(item) => {
                match item {
                    Ok(message) => Some(message),
                    Err(_) => None
                }
            },
            None => {None}
        }
            
    } 
}


