use std::f64::consts::PI;

use sipal::position::Position3;
use sipal::shader::ShaderProgram;
use sipal::triangle::Triangle;
use sipal::Window;
pub fn perspective(fov: f64, aspect_ratio: f64, near: f64, far: f64) -> glm::DMat4{
    let fov = fov * (PI/180.0);
    let tan_fov = (fov/2.0).tan();
    let range = far-near;
    glm::dmat4(
        1.0 / (tan_fov * aspect_ratio), 0.0, 0.0, 0.0,
        0.0, 1.0 / tan_fov, 0.0, 0.0,
        0.0, 0.0, -((far+near)/range), -((2.0*far*near)/range),
        0.0, 0.0, -1.0, 0.0
    )
}
pub fn y_rotation(angle: f64) -> glm::DMat4  {
    let angle = angle * (PI/180.0);
    glm::dmat4(
        angle.cos(), 0.0, angle.sin(), 0.0,
        0.0, 1.0, 0.0, 0.0,
        -angle.sin(), 0.0, angle.cos(), 0.0,
        0.0, 0.0, 0.0, 1.0,
    )
}
pub fn z_rotation(angle: f64) -> glm::DMat4  {
    let angle = angle * (PI/180.0);
    glm::dmat4(
        angle.cos(), -angle.sin(), 0.0,0.0,
        angle.sin(), angle.cos(), 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    )
}
pub fn translate(x: f64, y: f64, z: f64) -> glm::DMat4 {
    glm::transpose(&glm::dmat4(
        1.0,0.0, 0.0, x,
         0.0, 1.0, 0.0, y, 
         0.0, 0.0, 1.0, z, 
         0.0, 0.0, 0.0, 1.0
        ))
}
static mut Y_ROT: f64 = 0.0;
static mut Z_ROT: f64 = 0.0;
fn main() {
    let mut window =
    sipal::MiniFBWindow::create(800, 600, "Example with sidal").expect("Create window");
    let program = ShaderProgram {
        vertex_shader: &|pos| {
            let perspective_mat = perspective(70.0, 800.0/600.0, -1.0, 1000.0);
            let rotation_matrix = y_rotation(unsafe {
                Y_ROT
            })* z_rotation(unsafe {
                Z_ROT
            });
            let pos_vec = glm::dvec4(pos.0, pos.1, pos.2, 1.0);
            let result = perspective_mat * (translate(0.0, 0.0, -4.0) * (rotation_matrix * pos_vec));
            println!("{} {}", result.z, result.w);
            (result.x, result.y, result.z, result.w)
        },
        fragment_shader: &move |pos| {
            let fz = (pos.2/2.0)+0.5;
            (1.0, 1.0, fz, 1.0)
        },
    };
    while !window.should_close() {
        let context = window.context();
        context.use_shader(program);
        context.clear_color_buffer();
        context.clear_depth_buffer();
        context.draw_triangles(&[
            // FRONT
            Triangle(
                Position3(-0.5, 0.5, 0.0),
                Position3(0.5, -0.5, 0.0),
                Position3(-0.5, -0.5, 0.0),
            ),
            Triangle(
                Position3(0.5, 0.5, 0.0),
                Position3(0.5, -0.5, 0.0),
                Position3(-0.5, 0.5, 0.0),
            ),
            // BACK
            Triangle(
                Position3(0.5, -0.5, -1.0),
                Position3(-0.5, -0.5, -1.0),
                Position3(-0.5, 0.5, -1.0),
            ),
            Triangle(
                Position3(0.5, -0.5, -1.0),
                Position3(-0.5, 0.5, -1.0),
                Position3(0.5, 0.5, -1.0),
            ),

            // West
            Triangle(
                Position3(-0.5, 0.5, -0.0),
                Position3(-0.5, 0.5, -1.0),
                Position3(-0.5, -0.5, -1.0),
            ),
            Triangle(
                Position3(-0.5, -0.5, -1.0),
                Position3(-0.5, -0.5, -0.0),
                Position3(-0.5, 0.5, -0.0),
            ),
            // East
            Triangle(
                Position3(0.5, 0.5, -1.0),
                Position3(0.5, -0.5, -1.0),
                Position3(0.5, 0.5, 0.0),
            ),
            Triangle(
                Position3(0.5, -0.5, 0.0),
                Position3(0.5, 0.5, 0.0),
                Position3(0.5, -0.5, -1.0),
            ),

            // Top
            Triangle(
                Position3(-0.5, 0.5, -1.0),
                Position3(-0.5, 0.5, -0.0),
                Position3(0.5, 0.5, -1.0),
            ),
            Triangle(
                Position3(0.5, 0.5, -0.0),
                Position3(0.5, 0.5, -1.0),
                Position3(-0.5, 0.5, -0.0),
            ),

            // Bottom
            Triangle(
                Position3(-0.5, -0.5, -0.0),
                Position3(0.5, -0.5, -1.0),
                Position3(-0.5, -0.5, -1.0),
            ),
            Triangle(
                Position3(0.5, -0.5, -1.0),
                Position3(-0.5, -0.5, -0.0),
                Position3(0.5, -0.5, -0.0),
            ),
        ]);
        unsafe {
            Y_ROT += 1.0;
            Z_ROT += 1.0;
        }
        window.swap_buffers();
    }
}
