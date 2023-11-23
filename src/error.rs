use std::fmt::{self, Display, Formatter};

use x11rb::errors::{LibxcbLoadError, ConnectError, ReplyOrIdError, ReplyError, ConnectionError};

#[derive(Debug)]
pub enum Error {
    #[cfg(unix)]
    XcbLoadFailed(LibxcbLoadError),

    #[cfg(unix)]
    X11ConnectionFailed(ConnectError),

    #[cfg(unix)]
    X11AtomFetchFailed(ConnectionError),

    #[cfg(unix)]
    X11AtomReplyError(ReplyError),

    #[cfg(unix)]
    X11GenerateIdFailed(ReplyOrIdError),

    #[cfg(unix)]
    X11CreateWindowFailed(ConnectionError),

    #[cfg(unix)]
    X11SetTitleFailed(ConnectionError),

    #[cfg(unix)]
    X11WindowCloseHookFailed(ConnectionError),

    #[cfg(unix)]
    X11SetSizeHintsFailed(ConnectionError),

    #[cfg(unix)]
    X11MapWindowFailed(ConnectionError),

    #[cfg(unix)]
    X11FlushFailed(ConnectionError),

    #[cfg(unix)]
    X11WaitForEventFailed(ConnectionError)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::XcbLoadFailed(err) => write!(f, "Failed to load libxcb ({err})"),
            Self::X11ConnectionFailed(err) => write!(f, "Failed to connect to X11 server ({err})"),
            Self::X11AtomFetchFailed(err) => write!(f, "Failed to get required atoms ({err})"),
            Self::X11AtomReplyError(err) => write!(f, "Atom reply error ({err})"),
            Self::X11GenerateIdFailed(err) => write!(f, "Failed to generate an X11 ID ({err})"),
            Self::X11CreateWindowFailed(err) => write!(f, "Failed to create window ({err})"),
            Self::X11SetTitleFailed(err) => write!(f, "Failed to set window title ({err})"),
            Self::X11WindowCloseHookFailed(err) => write!(f, "Failed to hook window close event ({err})"),
            Self::X11SetSizeHintsFailed(err) => write!(f, "Failed to set window size hints ({err})"),
            Self::X11MapWindowFailed(err) => write!(f, "Failed to map window ({err})"),
            Self::X11FlushFailed(err) => write!(f, "Failed to flush X11 connection ({err})"),
            Self::X11WaitForEventFailed(err) => write!(f, "Failed to wait for event ({err})")
        }
    }
}

impl std::error::Error for Error {}