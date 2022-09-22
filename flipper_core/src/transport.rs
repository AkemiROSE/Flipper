use anyhow::{anyhow, Result};
use bytes::{Buf, Bytes, BytesMut};
use futures_util::{SinkExt, StreamExt};
use std::marker::Unpin;
use std::sync::Arc;
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufStream},
    net::TcpStream,
    sync::RwLock,
};
use tokio_stream::Stream;
use tokio_util::codec::Framed;

use crate::codec::{FramedCodec, MessageCodec};
use crate::message::{Message, MessageEntry};

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
            Some(item) => match item {
                Ok(message) => Some(message),
                Err(_) => None,
            },
            None => None,
        };
    }
}
