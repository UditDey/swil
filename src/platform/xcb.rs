use std::cell::Cell;
use std::ptr::NonNull;
use std::num::NonZeroU32;

use x11rb::{
    atom_manager,
    connection::Connection,
    xcb_ffi::{self, XCBConnection},
    wrapper::ConnectionExt as _,
    properties::WmSizeHints,
    protocol::{
        Event as X11Event,
        xproto::{
            self,
            ConnectionExt as _,
            WindowClass,
            CreateWindowAux,
            PropMode,
            EventMask
        }
    }
};

use raw_window_handle::{
    XcbWindowHandle, XcbDisplayHandle,
    WindowHandle, RawWindowHandle,
    DisplayHandle, RawDisplayHandle
};

use crate::{
    WindowConfig,
    Error,
    dpi::{Size, PhysicalSize, PhysicalPosition},
    event::{Event, KeyboardInput, MouseInput, MouseButton, ButtonState, MouseScroll}
};

atom_manager! {
    pub AtomSet: AtomSetCookie {
        WM_PROTOCOLS,
        WM_DELETE_WINDOW,
    }
}

pub struct Window {
    conn: XCBConnection,
    screen: i32,
    window: u32,
    scale_factor: f32,
    atoms: AtomSet,
    size: Cell<PhysicalSize>
}

impl Window {
    pub fn new(config: &WindowConfig) -> Result<Self, Error> {
        // Load libxcb
        xcb_ffi::load_libxcb().map_err(|err| Error::XcbLoadFailed(err))?;

        // Connect to X11 server
        let (conn, screen_num) = XCBConnection::connect(None).map_err(|err| Error::X11ConnectionFailed(err))?;
        let screen = &conn.setup().roots[screen_num];

        // Get needed atoms
        let atoms = AtomSet::new(&conn)
            .map_err(|err| Error::X11AtomFetchFailed(err))?
            .reply()
            .map_err(|err| Error::X11AtomReplyError(err))?;

        // Get scale factor
        // Try to get Xft.dpi
        let xft_dpi = x11rb::resource_manager::new_from_default(&conn)
            .ok()
            .map(|db| db.get_value::<u32>("Xft.dpi", "").ok())
            .flatten()
            .flatten();

        let scale_factor = match xft_dpi {
            Some(dpi) => dpi as f32 / 96.0,

            None => {
                // Calculate screen's physical DPI
                // This approach is taken from winit: https://github.com/rust-windowing/winit/blob/7bed5eecfdcbde16e5619fd137f0229e8e7e8ed4/src/platform_impl/linux/x11/util/randr.rs#L16
                let ppmm = (
                    (screen.width_in_pixels as f32 * screen.height_in_pixels as f32) /
                    (screen.width_in_millimeters as f32 * screen.height_in_millimeters as f32)
                ).sqrt();

                // Quantize 1/12 step size
                let dpi_factor = ((ppmm * (12.0 * 25.4 / 96.0)).round() / 12.0).max(1.0);

                if dpi_factor <= 20.0 {
                    dpi_factor
                } else {
                    1.0
                }
            }
        };

        // Calculate window physical size
        let size = match &config.size {
            Size::Logical(size) => size.to_physical(scale_factor),
            Size::Physical(size) => size.clone()
        };

        // Create window
        let window = conn.generate_id().map_err(|err| Error::X11GenerateIdFailed(err))?;

        let event_mask = EventMask::KEY_PRESS |
                         EventMask::KEY_RELEASE |
                         EventMask::ENTER_WINDOW |
                         EventMask::LEAVE_WINDOW |
                         EventMask::POINTER_MOTION |
                         EventMask::BUTTON_PRESS |
                         EventMask::BUTTON_RELEASE |
                         EventMask::RESIZE_REDIRECT |
                         EventMask::FOCUS_CHANGE;

        let aux = CreateWindowAux::new()
            .background_pixel(screen.black_pixel)
            .event_mask(event_mask);

        conn.create_window(
            x11rb::COPY_DEPTH_FROM_PARENT,
            window,
            screen.root,
            0,
            0,
            size.width as u16,
            size.height as u16,
            0,
            WindowClass::INPUT_OUTPUT,
            0,
            &aux
        ).map_err(|err| Error::X11CreateWindowFailed(err))?;

        // Set title
        conn.change_property8(
            PropMode::REPLACE,
            window,
            xproto::AtomEnum::WM_NAME,
            xproto::AtomEnum::STRING,
            config.title.as_bytes()
        ).map_err(|err| Error::X11SetTitleFailed(err))?;

        // Hook window close event
        conn.change_property32(
            PropMode::REPLACE,
            window,
            atoms.WM_PROTOCOLS,
            xproto::AtomEnum::ATOM,
            &[atoms.WM_DELETE_WINDOW]
        ).map_err(|err| Error::X11WindowCloseHookFailed(err))?;

        // Set size hints if not resizable
        if !config.resizable {
            let hints = WmSizeHints {
                min_size: Some((size.width as i32, size.height as i32)),
                max_size: Some((size.width as i32, size.height as i32)),
                ..Default::default()
            };

            hints
                .set_normal_hints(&conn, window)
                .map_err(|err| Error::X11SetSizeHintsFailed(err))?;
        }

        // Show window if needed
        if config.visible {
            conn.map_window(window).map_err(|err| Error::X11MapWindowFailed(err))?;
        }

        conn.flush().map_err(|err| Error::X11FlushFailed(err))?;

        let screen = screen.root as i32;

        Ok(Self {
            conn,
            screen,
            window,
            scale_factor,
            atoms,
            size: Cell::new(size)
        })
    }

    pub fn set_title(&self, title: &str) -> Result<(), Error> {
        self.conn.change_property8(
            PropMode::REPLACE,
            self.window,
            xproto::AtomEnum::WM_NAME,
            xproto::AtomEnum::STRING,
            title.as_bytes()
        ).map_err(|err| Error::X11SetTitleFailed(err))?;

        self.conn.flush().map_err(|err| Error::X11FlushFailed(err))
    }

    pub fn set_visible(&self, visible: bool) -> Result<(), Error> {
        if visible {
            self.conn.map_window(self.window).map_err(|err| Error::X11MapWindowFailed(err))?;
        }
        else {
            self.conn.unmap_window(self.window).map_err(|err| Error::X11MapWindowFailed(err))?;
        }

        self.conn.flush().map_err(|err| Error::X11FlushFailed(err))
    }

    pub fn set_resizable(&self, resizable: bool) -> Result<(), Error> {
        let hints = if !resizable {
            let size = self.size()?;

            WmSizeHints {
                min_size: Some((size.width as i32, size.height as i32)),
                max_size: Some((size.width as i32, size.height as i32)),
                ..Default::default()
            }
        }
        else {
            WmSizeHints {
                min_size: None,
                max_size: None,
                ..Default::default()
            }
        };

        hints
            .set_normal_hints(&self.conn, self.window)
            .map_err(|err| Error::X11SetSizeHintsFailed(err))?;

        self.conn.flush().map_err(|err| Error::X11FlushFailed(err))
    }

    pub fn size(&self) -> Result<PhysicalSize, Error> {
        let size = self.size.replace(PhysicalSize { width: 0, height: 0 });
        self.size.set(size.clone());

        Ok(size)
    }

    pub fn scale_factor(&self) -> f32 {
        self.scale_factor
    }

    pub fn window_handle(&self) -> WindowHandle {
        let window = NonZeroU32::new(self.window).unwrap();
        let handle = RawWindowHandle::Xcb(XcbWindowHandle::new(window));

        unsafe { WindowHandle::borrow_raw(handle) }
    }

    pub fn display_handle(&self) -> DisplayHandle {
        let conn = NonNull::new(self.conn.get_raw_xcb_connection());
        let handle = RawDisplayHandle::Xcb(XcbDisplayHandle::new(conn, self.screen));

        unsafe { DisplayHandle::borrow_raw(handle) }
    }

    pub fn event_loop(&self, func: impl Fn(Event, &mut bool)) -> Result<(), Error> {
        loop {
            let x11_event = self.conn.wait_for_event().map_err(|err| Error::X11WaitForEventFailed(err))?;

            let event = match x11_event {
                X11Event::ResizeRequest(event) => {
                    let new_size = PhysicalSize { width: event.width as u32, height: event.height as u32 };
                    self.size.set(new_size.clone());

                    Some(Event::Resized(new_size))
                },
                
                X11Event::ClientMessage(event) => {
                    let data = event.data.as_data32();

                    if event.format == 32 && data[0] == self.atoms.WM_DELETE_WINDOW {
                        Some(Event::CloseRequested)
                    }
                    else {
                        None
                    }
                },

                X11Event::FocusIn(_) => Some(Event::FocusChanged(true)),
                X11Event::FocusOut(_) => Some(Event::FocusChanged(false)),

                X11Event::KeyPress(event) => {
                    Some(Event::KeyboardInput(KeyboardInput {
                        code: event.detail,
                        state: ButtonState::Pressed,
                        text: None
                    }))
                },

                X11Event::KeyRelease(event) => {
                    Some(Event::KeyboardInput(KeyboardInput {
                        code: event.detail,
                        state: ButtonState::Released,
                        text: None
                    }))
                },

                X11Event::MotionNotify(event) => Some(Event::CursorMoved(PhysicalPosition { x: event.event_x as u32, y: event.event_y as u32 })),

                X11Event::LeaveNotify(_) => Some(Event::CursorLeft),
                X11Event::EnterNotify(_) => Some(Event::CursorEntered),

                X11Event::ButtonPress(event) => map_button_event(event.detail, ButtonState::Pressed),
                X11Event::ButtonRelease(event) => map_button_event(event.detail, ButtonState::Released),

                _ => None
            };

            if let Some(event) = event {
                let mut exit = false;
                func(event, &mut exit);

                if exit {
                    return Ok(())
                }
            }
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.conn.destroy_window(self.window).unwrap();
    }
}

fn map_button_event<'a>(button: u8, state: ButtonState) -> Option<Event<'a>> {
    match button {
        1 => Some(Event::MouseInput(MouseInput { button: MouseButton::Left, state })),
        2 => Some(Event::MouseInput(MouseInput { button: MouseButton::Middle, state })),
        3 => Some(Event::MouseInput(MouseInput { button: MouseButton::Right, state })),

        4 => if let ButtonState::Pressed = state {
            Some(Event::MouseScroll(MouseScroll::Up))
        }
        else {
            None
        },

        5 => if let ButtonState::Pressed = state {
            Some(Event::MouseScroll(MouseScroll::Down))
        }
        else {
            None
        },

        6 => if let ButtonState::Pressed = state {
            Some(Event::MouseScroll(MouseScroll::Left))
        }
        else {
            None
        },

        7 => if let ButtonState::Pressed = state {
            Some(Event::MouseScroll(MouseScroll::Right))
        }
        else {
            None
        },

        other => Some(Event::MouseInput(MouseInput { button: MouseButton::Other(other), state }))
    }
}