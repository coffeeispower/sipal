use crate::position::Position3;
#[derive(Copy, Clone)]
pub struct ShaderProgram<'f, 'v> {
    pub fragment_shader: &'f dyn Fn(Position3) -> (f64, f64, f64, f64),
    pub vertex_shader: &'v dyn Fn(Position3) -> (f64, f64, f64, f64),
}
