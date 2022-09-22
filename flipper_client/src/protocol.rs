use bytes::{BufMut, Buf};
use anyhow::{Result, anyhow};

use crate::message::Message;

pub struct BinaryProtocol<T> {
    buf: T
}

impl<T> BinaryProtocol<T> {
    pub fn new(buf: T) -> Self {
        Self { buf }
    }
}

impl<T: Buf> BinaryProtocol<T> {
    #[inline]
    pub fn read_bytes(&mut self) -> Result<Vec<u8>> {
        protocol_len_check(&self.buf, 4)?;
        let num_bytes = self.buf.get_i32() as usize;
        let mut output = vec![0; num_bytes];
        protocol_len_check(&self.buf, num_bytes)?;
        self.buf.copy_to_slice(&mut output);
        Ok(output)
    }

    #[inline]
    pub fn read_byte(&mut self) -> Result<u8> {
        protocol_len_check(&self.buf, 1)?;
        Ok(self.buf.get_u8())
    }

    #[inline]
    pub fn read_i32(&mut self) -> Result<i32> {
        protocol_len_check(&self.buf, 4)?;
        Ok(self.buf.get_i32())
    }

}

#[inline]
fn protocol_len_check<T>(buf: &T, required_len: usize) -> Result<()>
where
    T: bytes::Buf,
{
    #[cfg(not(feature = "unstable"))]
    if buf.remaining() >= required_len {
        return Ok(());
    }
    #[cfg(feature = "unstable")]
    if std::intrinsics::likely(buf.remaining() >= required_len) {
        return Ok(());
    }
    Err(anyhow!("unexpected data length"))
}
