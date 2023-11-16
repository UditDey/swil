use std::error::Error;
use std::fmt::{self, Display, Formatter};

use xcb::ConnError;

#[derive(Debug)]
pub enum WindowCreationError {
    #[cfg(unix)]
    XcbConnectionFailed(ConnError),

    #[cfg(unix)]
    XcbScreenNotFound,

    #[cfg(unix)]
    XcbFlushFailed(ConnError),

    #[cfg(unix)]
    XcbAtomInternFailed(xcb::Error)
}

impl Display for WindowCreationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::XcbConnectionFailed(err) => write!(f, "XCB connection failed ({err})"),
            Self::XcbScreenNotFound => write!(f, "Could not find screen specified by the X11 server"),
            Self::XcbFlushFailed(err) => write!(f, "XCB connection flush failed ({err})"),
            Self::XcbAtomInternFailed(err) => write!(f, "Failed to intern X11 atoms ({err})")
        }
    }
}

impl Error for WindowCreationError {}

#[derive(Debug)]
pub enum EventLoopError {
    #[cfg(unix)]
    XcbWaitForEventFailed(xcb::Error)
}

impl Display for EventLoopError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::XcbWaitForEventFailed(err) => write!(f, "XCB wait_for_event() failed ({err})")
        }
    }
}

impl Error for EventLoopError {}