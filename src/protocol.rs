use bytes::{BufMut, Buf};
use anyhow::Result;

use crate::message::Message;

pub struct BinaryProtocol<T> {
    buf: T
}

impl<T> BinaryProtocol<T> {
    pub fn new(buf: T) -> Self {
        Self { buf }
    }
}

impl<T: BufMut> BinaryProtocol<T> {
    #[inline]
    pub fn write_bytes(&mut self, b: &[u8]) -> Result<()> {
        self.write_u64(b.len() as u64)?;
        self.buf.put_slice(b);
        Ok(())
    }

    #[inline]
    pub fn write_byte(&mut self, b: u8) -> Result<()> {
        self.buf.put_u8(b);
        Ok(())
    }

    #[inline]
    pub fn write_u64(&mut self, i: u64) -> Result<()> {
        self.buf.put_u64(i);
        Ok(())
    } 

}