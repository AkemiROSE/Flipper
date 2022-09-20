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
use anyhow::Result;

use crate::message::{Message, MessageEntry};
use crate::codec::MessageCodec;

pub struct Transport<T: AsyncRead + AsyncWrite + Unpin>(Framed<T, MessageCodec>);


impl<T: AsyncRead + AsyncWrite + Unpin> Transport<T> {
    pub fn new(io: T) -> Self {
        let codec = MessageCodec::default();
        Transport(Framed::new(io, codec))
    } 

    pub async fn send(&mut self, msg: MessageEntry) -> Result<()> {
        self.0.send(msg).await?;
        Ok(())
    }
}


