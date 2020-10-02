use super::program::ShaderData;
use crate::engine::base::*;

#[derive(Debug)]
pub struct Frame {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<PixelBuffer>,
}

#[derive(Debug)]
pub struct PixelBuffer {
    pub coord: Vec2,
    pub color: (u8, u8, u8, u8),
    pub z: u8, //z buffer
    pub varying: Option<Vec<ShaderData>>,
}

impl Frame {
    pub fn clear(&mut self) {
        for i in 0..self.buffer.len() {
            self.buffer[i].color = (0, 0, 0, 0);
            self.buffer[i].varying = None;
            self.buffer[i].z = 0;
        }
    }
    pub fn get(&self, coord: &(usize, usize)) -> Option<&PixelBuffer> {
        if coord.0 >= self.width || coord.1 >= self.height {
            None
        } else {
            self.buffer.get(coord.1 * self.width + coord.0)
        }
    }
    pub fn get_mut(&mut self, coord: &(usize, usize)) -> Option<&mut PixelBuffer> {
        if coord.0 >= self.width || coord.1 >= self.height {
            None
        } else {
            self.buffer.get_mut(coord.1 * self.width + coord.0)
        }
    }
    pub fn new(width: usize, height: usize) -> Self {
        let mut buffer = vec![];
        for y in 0..height {
            for x in 0..width {
                buffer.push(PixelBuffer {
                    coord: Vec2::new(x as f64, y as f64),
                    color: (0, 0, 0, 0),
                    z: 0,
                    varying: None,
                })
            }
        }
        Frame {
            buffer,
            width,
            height,
        }
    }
}
