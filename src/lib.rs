extern crate core;

mod context;
#[cfg(feature = "minifb_window")]
mod minifb_window_impl;
pub mod position;
pub mod shader;
pub mod triangle;

pub use context::Context;
#[cfg(feature = "minifb_window")]
pub use minifb_window_impl::*;

pub trait Window
where
    Self: Sized,
{
    type CreationError;
    fn create(width: usize, height: usize, title: &str) -> Result<Self, Self::CreationError>;
    fn context(&mut self) -> &mut Context;
    fn should_close(&self) -> bool;
    fn set_should_close(&mut self, should_close: bool);
    fn swap_buffers(&mut self);
    // TODO: Events
}
