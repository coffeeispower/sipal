use minifb::WindowOptions;

use crate::context::Context;
use crate::Window;

pub struct MiniFBWindow {
    context: Context,
    window: minifb::Window,
    set_should_close: bool,
}

impl Window for MiniFBWindow {
    type CreationError = String;
    fn create(width: usize, height: usize, title: &str) -> Result<Self, String> {
        let window = minifb::Window::new(title, width, height, WindowOptions::default()).map_err(
            |e| match e {
                minifb::Error::WindowCreate(msg) => msg,
                _ => unreachable!(),
            },
        )?;

        Ok(Self {
            window,
            context: Context::create(width, height),
            set_should_close: false,
        })
    }
    fn context(&mut self) -> &mut Context {
        &mut self.context
    }
    fn should_close(&self) -> bool {
        !self.window.is_open() || self.set_should_close
    }
    fn set_should_close(&mut self, should_close: bool) {
        self.set_should_close = should_close;
    }
    fn swap_buffers(&mut self) {
        let (width, height) = self.window.get_size();
        self.window
            .update_with_buffer(&self.context.backbuffer, width, height)
            .expect("Failed to swap buffers");
    }
}
