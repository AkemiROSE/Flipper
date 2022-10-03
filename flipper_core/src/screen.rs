use std::io::ErrorKind::WouldBlock;
use std::io::Result;
use std::thread;

use scrap::{Capturer, Display, Frame};

const FPS: u32 = 10;
pub struct ScreenCap {
    capturer: Capturer,
}

impl ScreenCap {
    pub fn new() -> Result<Self> {
        let display = Display::primary()?;
        let mut capturer = Capturer::new(display)?;
        Ok(Self { capturer })
    }

    pub fn window_size(&self) -> (usize, usize) {
        (self.capturer.width(), self.capturer.height())
    }

    pub async fn capture(&mut self) -> Result<Vec<u8>> {
        let (width, height) = self.window_size();
        let one_second = std::time::Duration::new(1, 0);
        let one_frame = one_second / FPS;
        loop {
            match self.capturer.frame() {
                Ok(buffer) => {
                    //return Ok(frame_to_bytes(buffer, width, height));
                    return Ok(buffer.to_vec());
                }
                Err(error) => {
                    thread::sleep(one_frame);
                    if error.kind() == WouldBlock {                    
                        continue;
                    } else {
                        return Err(error);
                    }
                }
            };
        }
    }
}

fn frame_to_bytes(buffer: Frame, width: usize, height: usize) -> Vec<u8> {
    let mut bitflipped = Vec::with_capacity(width * height * 4);
    let stride = buffer.len() / height;
    for y in 0..height {
        for x in 0..width {
            let i = stride * y + 4 * x;
            bitflipped.extend_from_slice(&[buffer[i + 2], buffer[i + 1], buffer[i], 255]);
        }
    }
    bitflipped
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_slice_frame() -> Result<()> {
        let mut scap = ScreenCap::new()?;
        let (width, height) = scap.window_size();
        let mut send_frame = vec![0u8; width * height * 4];
        let mut recv_frame = vec![0u8; width * height * 4];
        for _ in 0..10 {
            let mut new_frame = scap.capture()?;
            if send_frame.eq(&new_frame) {
                continue;
            }
            let saved_frame = new_frame.clone();
            new_frame
                .iter_mut()
                .zip(send_frame.iter())
                .for_each(|(b1, b2)| {
                    *b1 ^= *b2;
                });
            send_frame = saved_frame.clone();
            recv_frame
                .iter_mut()
                .zip(new_frame.iter())
                .for_each(|(b1, b2)| {
                    *b1 ^= *b2;
                });
            assert!(recv_frame.eq(&saved_frame));
        }
        Ok(())
    }
}
