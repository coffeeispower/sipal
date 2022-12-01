use bitflags::bitflags;
use minifb::clamp;

use crate::position::{
    normalized_to_real, position_to_index, real_to_normalized, to_1d_index, Position2, Position3,
};
use crate::shader::ShaderProgram;
use crate::triangle::Triangle;

bitflags! {
    pub struct Features: u16 {
        const DEPTH_TESTING = 0b1;
    }
}

pub struct Context<'v, 'f> {
    pub(crate) backbuffer: Vec<u32>,
    features: Features,
    depth_buffer: Vec<f64>,
    width: usize,
    height: usize,
    shader: Box<ShaderProgram<'v, 'f>>,
}

impl<'v, 'f> Context<'v, 'f> {
    pub fn create(width: usize, height: usize) -> Self {
        Self {
            backbuffer: vec![0; width * height],
            features: Features::empty(),
            width,
            height,
            depth_buffer: vec![f64::INFINITY; width * height],
            shader: Box::new(ShaderProgram {
                vertex_shader: &|vertex_position| {
                    (vertex_position.0, vertex_position.1, vertex_position.2, 1.0)
                },
                fragment_shader: &|_frag_position| (1.0, 1.0, 1.0, 1.0),
            }),
        }
    }
    pub fn enable(&mut self, features: Features) {
        self.features |= features;
    }
    pub fn disable(&mut self, features: Features) {
        self.features &= !features;
    }
    pub fn use_shader(&mut self, shader: ShaderProgram<'v, 'f>) {
        self.shader = Box::new(shader);
    }
    fn run_vertex_shader(&self, pos: &mut Position3) {
        let result = (*self.shader.vertex_shader)(*pos);
        if result.3 == 0.0 {
            panic!("W component cannot be 0");
        }
        pos.0 = result.0 / result.3;
        pos.1 = result.1 / result.3;
        pos.2 = result.2 / result.3;
    }
    pub fn draw_triangle(&mut self, mut triangle: Triangle) {
        self.run_vertex_shader(&mut triangle.0);
        self.run_vertex_shader(&mut triangle.1);
        self.run_vertex_shader(&mut triangle.2);
        fn get_weight(triangle: Triangle, position: Position2) -> (f64, f64, f64) {
            let x1 = triangle.0 .0;
            let x2 = triangle.1 .0;
            let x3 = triangle.2 .0;
            let y1 = triangle.0 .1;
            let y2 = triangle.1 .1;
            let y3 = triangle.2 .1;
            let px = position.0;
            let py = position.1;
            let w1 = (((y2 - y3) * (px - x3)) + ((x3 - x2) * (py - y3)))
                / (((y2 - y3) * (x1 - x3)) + ((x3 - x2) * (y1 - y3)));
            let w2 = (((y3 - y1) * (px - x3)) + ((x1 - x3) * (py - y3)))
                / (((y2 - y3) * (x1 - x3)) + ((x3 - x2) * (y1 - y3)));
            let w3 = 1.0 - w1 - w2;
            (w1, w2, w3)
        }
        let real_triangle = Triangle(
            normalized_to_real(triangle.0, self.width, self.height).into(),
            normalized_to_real(triangle.1, self.width, self.height).into(),
            normalized_to_real(triangle.2, self.width, self.height).into(),
        );
        for y in 0..self.height {
            for x in 0..self.width {
                let (w0, w1, w2) = get_weight(real_triangle, Position2(x as f64, y as f64));
                let fx = (w0 * triangle.0 .0) + (w1 * triangle.1 .0) + (w2 * triangle.2 .0);
                let fy = (w0 * triangle.0 .1) + (w1 * triangle.1 .1) + (w2 * triangle.2 .1);
                let fz = (w0 * triangle.0 .2) + (w1 * triangle.1 .2) + (w2 * triangle.2 .2);
                if fz > -1.0
                    && fz < 1.0
                    && real_triangle.contains_point(Position2(x as f64, y as f64))
                {
                    let mut color = (*self.shader.fragment_shader)(Position3(fx, fy, fz));
                    color.0 = clamp(0.0, color.0, 1.0);
                    color.1 = clamp(0.0, color.1, 1.0);
                    color.2 = clamp(0.0, color.2, 1.0);
                    color.3 = clamp(0.0, color.3, 1.0);
                    let r = (255.0 * color.0) as u32;
                    let g = (255.0 * color.1) as u32;
                    let b = (255.0 * color.2) as u32;
                    self.backbuffer[to_1d_index(x, y, self.width)] = r | (g << 8) | (b << 16);
                }
            }
        }
    }
    pub fn draw_triangles(&mut self, triangles: &[Triangle]) {
        for t in triangles {
            self.draw_triangle(*t);
        }
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
