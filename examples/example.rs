use sipal::Window;

fn main() {
    let mut window =
        sipal::MiniFBWindow::create(500, 500, "Example with sidal").expect("Create window");
    let context = window.context();
    while !window.should_close() {
        window.swap_buffers();
    }
}
