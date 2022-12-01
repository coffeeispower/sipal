use sipal::position::Position3;
use sipal::shader::ShaderProgram;
use sipal::triangle::Triangle;
use sipal::Window;

fn main() {
    let mut window =
        sipal::MiniFBWindow::create(800, 600, "Example with sidal").expect("Create window");
    let program = ShaderProgram {
        vertex_shader: &|pos| {
            (pos.0, pos.1, pos.2, 1.0)
        },
        fragment_shader: &|pos| {
            (pos.0, pos.1, pos.2, 1.0)
        }
    };
    while !window.should_close() {
        let context = window.context();
        context.use_shader(program);
        context.clear_color_buffer();
        context.clear_depth_buffer();
        context.draw_triangles(&[
            Triangle(
                Position3(-0.5, 0.5, 0.5),
                Position3(0.5, 0.5, 0.5),
                Position3(-0.5, -0.5, 0.5),
            ),
            Triangle(
                Position3(0.5, 0.5, 0.5),
                Position3(0.5, -0.5, 0.5),
                Position3(-0.5, -0.5, 0.5),
            ),
        ]);
        window.swap_buffers();
    }
}
