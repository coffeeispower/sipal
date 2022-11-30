use crate::position::{
    normalized_to_real, position_to_index, real_to_normalized, to_1d_index, Position2, Position3,
};
use crate::triangle::Triangle;
use bitflags::bitflags;
bitflags! {
    pub struct Features: u16 {
        const DEPTH_TESTING = 0b1;
    }
}
pub struct Context {
    pub(crate) backbuffer: Vec<u32>,
    features: Features,
    depth_buffer: Vec<f64>,
    width: usize,
    height: usize,
}

impl Context {
    pub fn create(width: usize, height: usize) -> Self {
        Self {
            backbuffer: vec![0; width * height],
            features: Features::empty(),
            width,
            height,
            depth_buffer: vec![0.0; width * height],
        }
    }
    pub fn enable(&mut self, features: Features) {
        self.features |= features;
    }
    pub fn disable(&mut self, features: Features) {
        self.features &= !features;
    }

    pub fn draw_triangle(&mut self, triangle: Triangle) {
        let real_triangle = Triangle(
            normalized_to_real(triangle.0, self.width, self.height).into(),
            normalized_to_real(triangle.1, self.width, self.height).into(),
            normalized_to_real(triangle.2, self.width, self.height).into(),
        );
        for y in 0..self.height {
            for x in 0..self.width {
                if real_triangle.contains_point(Position2(x as f64, y as f64)) {
                    self.backbuffer[to_1d_index(x, y, self.width)] = 0xffffff;
                }
            }
        }
        self.backbuffer[position_to_index(triangle.0, self.width, self.height)] = 0xffff00;
        self.backbuffer[position_to_index(triangle.1, self.width, self.height)] = 0xffff00;
        self.backbuffer[position_to_index(triangle.2, self.width, self.height)] = 0xffff00;
    }
    pub fn clear_depth_buffer(&mut self) {
        for i in self.depth_buffer.iter_mut() {
            *i = 0.0;
        }
    }
    pub fn clear_color_buffer(&mut self) {
        for i in self.backbuffer.iter_mut() {
            *i = 0;
        }
    }
}
