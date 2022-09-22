use flate2::Compression;
use std::io::{self, prelude::*};

use flate2::{read::ZlibDecoder, write::ZlibEncoder};

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

pub fn frame_to_img_bytes(frame: &[u8], width: usize, height: usize) -> Vec<u8> {
    let mut bitflipped = Vec::with_capacity(width * height * 4);
    let stride = frame.len() / height;
    for y in 0..height {
        for x in 0..width {
            let i = stride * y + 4 * x;
            bitflipped.extend_from_slice(&[frame[i + 2], frame[i + 1], frame[i], 255]);
        }
    }
    bitflipped
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let mut s = vec![112, 34, 56, 67, 78, 89, 45, 34, 34, 23, 23, 12, 56];
        let compressed = compress(&s[..]).unwrap();
        let res = decompress(&compressed[..]).unwrap();
        assert!(s.eq(&res))
    }
}
