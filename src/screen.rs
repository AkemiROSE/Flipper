use std::io::Result;
use std::thread;
use std::io::ErrorKind::WouldBlock;


use scrap::{Capturer, Display, Frame};

const FPS: u32 = 100;
pub struct ScreenCap {
    capturer: Capturer,

}

impl ScreenCap {
    pub fn new() -> Result<Self> {
        let display = Display::primary()?;
        let mut capturer = Capturer::new(display)?;
        Ok(Self { capturer})
    }

    pub fn window_size(&self) -> (usize, usize) {
        (self.capturer.width(), self.capturer.height())
    }

    pub fn capture(&mut self) -> Result<Vec<u8>> {
        let (width, height) = self.window_size();
        let one_second = std::time::Duration::new(1, 0);
        let one_frame = one_second / FPS;
        loop{
            match self.capturer.frame() {
                Ok(buffer) => {
                    return Ok(frame_to_bytes(buffer, width, height));
                },
                Err(error) => {
                    if error.kind() == WouldBlock {
                        thread::sleep(one_frame);
                        continue;
                    } else {
                        return Err(error);
                    }
                }
            };        
        } 
    } 

}

fn frame_to_bytes(buffer: Frame, width: usize, height: usize)-> Vec<u8> {
    let mut bitflipped = Vec::with_capacity(width * height * 4);
        let stride = buffer.len() / height;
        for y in 0..height {
            for x in 0..width {
                let i = stride * y + 4 * x;
                bitflipped.extend_from_slice(&[
                    buffer[i + 2],
                    buffer[i + 1],
                    buffer[i],
                    255
                ]);
            }
        }
        bitflipped
}