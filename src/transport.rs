use std::sync::Arc;
use bytes::BytesMut;
use tokio::{
    net::TcpStream,
    io::{AsyncWriteExt, AsyncReadExt}, 
    sync::RwLock
};
use anyhow::Result;

use crate::protocol::BinaryOutputProtocol;
pub struct Transport {
    inner: TcpStream,
    protocol: BinaryOutputProtocol<BytesMut>,
}

impl Transport {
    pub fn new(tcp_stream: TcpStream) -> Self {
        Self {
            inner: tcp_stream,
            protocol: BinaryOutputProtocol::new(BytesMut::new())
        }
    }

    

    pub async fn send_all(&mut self, src: &[u8]) -> Result<()> {
        
        self.inner.write_all(src).await?;
        Ok(())
    }

    pub async fn recv_exact(&mut self, buf: &mut [u8]) -> Result<()> {
       
        self.inner.read_exact(buf).await?;
        Ok(())
    } 
}