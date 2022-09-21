use std::io::{self, prelude::*};
use flate2::Compression;

use flate2::{
    read::ZlibDecoder,
    write::ZlibEncoder
};

pub fn compress(bytes: &[u8]) -> io::Result<Vec<u8>> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(bytes)?;
    e.finish()
} 

pub fn decompress(bytes: &[u8]) -> io::Result<Vec<u8>> {
    let mut gz = ZlibDecoder::new(bytes);
    let mut buf = vec![];
    gz.read_to_end(&mut buf)?;
    Ok(buf)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let mut s = vec![112,34,56,67,78,89,45,34,34,23,23,12,56];
        let compressed = compress(&s[..]).unwrap();
        let res = decompress(&compressed[..]).unwrap();
        assert!(s.eq(&res))
    }
}