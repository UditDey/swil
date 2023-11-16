mod platform;

pub mod dpi;
pub mod error;

use raw_window_handle::RawWindowHandle;

use dpi::{EitherSize, LogicalSize, LogicalPosition};
use error::{WindowCreationError, EventLoopError};

#[derive(Debug)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Other(u8)
}

pub type Keycode = u16;

#[derive(Debug)]
pub enum Event {
    KeyPressed(Keycode),
    KeyReleased(Keycode),

    MouseEntered,
    MouseLeft,
    MouseMoved(LogicalPosition),
    MouseButtonPressed(MouseButton),
    MouseButtonReleased(MouseButton),

    Resized(LogicalSize),
    ShouldClose
}

pub struct EventInfo {
    pub event: Event,
    pub scale_factor: f32,
    pub exit: bool
}

pub struct WindowConfig<'a> {
    pub title: &'a str,
    pub size: EitherSize
}

impl<'a> Default for WindowConfig<'a> {
    fn default() -> Self {
        Self {
            title: "swil window",
            size: (750.0, 500.0).into()
        }
    }
}

pub struct Window {
    inner: platform::Window
}

impl Window {
    pub fn new(config: &WindowConfig) -> Result<Self, WindowCreationError> {
        platform::Window::new(config).map(|inner| Self { inner })
    }

    pub fn handle(&self) -> RawWindowHandle {
        self.inner.handle()
    }

    pub fn event_loop(&self, func: impl Fn(&mut EventInfo)) -> Result<(), EventLoopError> {
        self.inner.event_loop(func)
    }
}