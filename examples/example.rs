use sipal::position::Position3;
use sipal::triangle::Triangle;
use sipal::Window;

fn main() {
    let mut window =
        sipal::MiniFBWindow::create(500, 500, "Example with sidal").expect("Create window");
    while !window.should_close() {
        let context = window.context();
        context.draw_triangle(Triangle(
            Position3(-0.5, -0.5, 0.0),
            Position3(0.5, -0.5, 0.0),
            Position3(0.0, 0.5, 0.0),
        ));
        window.swap_buffers();
    }
}
