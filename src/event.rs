use crate::dpi::{PhysicalSize, PhysicalPosition};

/// The state of a button event
#[derive(Debug, Clone)]
pub enum ButtonState {
    Pressed,
    Released
}

pub type KeyCode = u8;

/// A keyboard input event
#[derive(Debug, Clone)]
pub struct KeyboardInput<'a> {
    pub code: KeyCode,
    pub state: ButtonState,
    pub text: Option<&'a str>
}

/// A mouse scroll event
#[derive(Debug, Clone)]
pub enum MouseWheelDelta {
    /// Amount in lines or rows to scroll in the horizontal and vertical directions
    /// 
    /// Positive values mean that the content should be scrolled right/down.
    /// This is produced by physical mouse wheels
    LineDelta(u32, u32),

    /// Amount in physical pixel units to scroll in the horizontal and vertical directions
    PixelDelta(PhysicalPosition)
}

/// A mouse button
#[derive(Debug, Clone)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Other(u8)
}

/// A mouse button input event
#[derive(Debug, Clone)]
pub struct  MouseInput {
    pub button: MouseButton,
    pub state: ButtonState
}

/// An event recieved by a window
#[derive(Debug, Clone)]
pub enum Event<'a> {
    /// Size of the window has changed
    /// 
    /// Contains the new physical size
    Resized(PhysicalSize),

    /// The window has been requested to close
    /// 
    /// This can happen due to clicking the close button, `alt + f4`, etc
    CloseRequested,

    /// There is a change in the window's focus
    /// 
    /// The value is true if focus has been gained, and false if it has
    /// been lost
    FocusChanged(bool),

    /// A keyboard event has occurred
    KeyboardInput(KeyboardInput<'a>),

    /// The cursor has moved inside the window
    /// 
    /// The value is the physical coordinates of the cursor, where (0, 0)
    /// is the top left corner of the window
    CursorMoved(PhysicalPosition),

    /// The cursor has entered the window
    CursorEntered,

    /// The cursor has left the window
    CursorLeft,

    /// A mouse wheel movement or touchpad scroll has occurred
    MouseWheel(MouseWheelDelta),

    /// A mouse button event has occurred
    MouseInput(MouseInput)
}