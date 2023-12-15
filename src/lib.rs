//! A simple cross-platform window library

mod platform;
mod error;

pub mod dpi;
pub mod event;

use raw_window_handle::{
    WindowHandle, DisplayHandle,
    HasWindowHandle, HasDisplayHandle,
    HandleError
};

pub use error::Error;

use dpi::{Size, LogicalSize, PhysicalSize};
use event::Event;

/// Initial configuration of a window
/// 
/// Provides a builder pattern to set configuration options
pub struct WindowConfig<'a> {
    pub(crate) title: &'a str,
    pub(crate) visible: bool,
    pub(crate) resizable: bool,
    pub(crate) size: Size
}

impl<'a> WindowConfig<'a> {
    /// Create a new config with default values
    pub fn new() -> Self {
        WindowConfig {
            title: "swil window",
            visible: true,
            resizable: true,
            size: Size::Logical(LogicalSize { width: 750.0, height: 500.0 })
        }
    }

    /// Sets the initial window title
    /// 
    /// The default value is "swil window"
    /// 
    /// This can later be changed with [`Window::set_title()`]
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }

    /// Sets whether the window will be visible when created
    /// 
    /// The default value is `true`
    /// 
    /// Its possible for the window to display garbage data when first created.
    /// To prevent that, set this to false and then make the window visible using
    /// [`Window::set_visible()`] when you are ready to render actual contents
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Sets whether the window will be resizable when created
    /// 
    /// The default is `true`
    /// 
    /// This can later be changed with [`Window::set_resizable()`]
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Sets the inital inner size of the window
    /// 
    /// The default value is `(600.0, 400.0)` logical units
    /// 
    /// This represents the inner size of the window, excluding the size of decorations
    /// 
    /// This can later be retrieved with [`Window::size()`]
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
}

/// A graphical window
pub struct Window {
    inner: platform::Window
}

impl Window {
    /// Create a new window
    /// 
    /// The given [`WindowConfig`] is used to set the initial configuration.
    /// The options can be changed later using the window's setter methods
    pub fn new(config: &WindowConfig) -> Result<Self, Error> {
        platform::Window::new(config).map(|inner| Self { inner })
    }

    /// Sets the window's title
    pub fn set_title(&self, title: &str) -> Result<(), Error> {
        self.inner.set_title(title)
    }

    /// Sets whether the window is visible
    pub fn set_visible(&self, visibile: bool) -> Result<(), Error> {
        self.inner.set_visible(visibile)
    }

    /// Sets whether the window is resizable
    pub fn set_resizable(&self, resizable: bool) -> Result<(), Error> {
        self.inner.set_resizable(resizable)
    }

    /// Gets the current inner size of the window
    pub fn size(&self) -> Result<PhysicalSize, Error> {
        self.inner.size()
    }

    /// Gets the window's current scale factor
    pub fn scale_factor(&self) -> f32 {
        self.inner.scale_factor()
    }

    /// Runs the window event loop
    /// 
    /// This function blocks the thread while waiting for new window events to
    /// be recieved. The given closure is called to process each event. The closure
    /// also gets a `exit: &mut bool` parameter, which is initially `false` and can
    /// be set to `true` by the closure to indicate that it wants to exit from the
    /// event loop
    pub fn event_loop(&self, func: impl Fn(Event, &mut bool)) -> Result<(), Error> {
        self.inner.event_loop(func)
    }
}

impl HasWindowHandle for Window {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        Ok(self.inner.window_handle())
    }
}

impl HasDisplayHandle for Window {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        Ok(self.inner.display_handle())
    }
}