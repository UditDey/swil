use std::num::NonZeroU32;

use xcb::{x, atoms_struct, Connection, Xid};
use raw_window_handle::{RawWindowHandle, XcbWindowHandle};

use crate::{
    Event,
    EventInfo,
    MouseButton,
    WindowConfig,
    dpi::{PhysicalPosition, PhysicalSize},
    error::{WindowCreationError, EventLoopError},
};

atoms_struct! {
    struct Atoms {
        wm_protocols => b"WM_PROTOCOLS",
        wm_del_window => b"WM_DELETE_WINDOW",
    }
}

pub struct Window {
    conn: Connection,
    window: x::Window,
    atoms: Atoms,
    scale_factor: f32
}

impl Window {
    pub fn new(config: &WindowConfig) -> Result<Self, WindowCreationError> {
        let (conn, screen_num) = Connection::connect(None).map_err(|err| WindowCreationError::XcbConnectionFailed(err))?;

        let setup = conn.get_setup();
        let window = conn.generate_id::<x::Window>();
        let screen = setup.roots().nth(screen_num as usize).ok_or(WindowCreationError::XcbScreenNotFound)?;

        let scale_factor = 1.0;
        let size = config.size.to_physical(scale_factor);

        let event_mask = x::EventMask::KEY_PRESS |
                         x::EventMask::KEY_RELEASE |
                         x::EventMask::ENTER_WINDOW |
                         x::EventMask::LEAVE_WINDOW |
                         x::EventMask::POINTER_MOTION |
                         x::EventMask::BUTTON_PRESS |
                         x::EventMask::BUTTON_RELEASE |
                         x::EventMask::RESIZE_REDIRECT;

        conn.send_request(&x::CreateWindow {
            depth: x::COPY_FROM_PARENT as u8,
            wid: window,
            parent: screen.root(),
            x: 0,
            y: 0,
            width: size.width as u16,
            height: size.height as u16,
            border_width: 10,
            class: x::WindowClass::InputOutput,
            visual: screen.root_visual(),
            value_list: &[
                x::Cw::BackPixel(screen.black_pixel()),
                x::Cw::EventMask(event_mask),
            ],
        });

        conn.send_request(&x::ChangeProperty {
            mode: x::PropMode::Replace,
            window,
            property: x::ATOM_WM_NAME,
            r#type: x::ATOM_STRING,
            data: config.title.as_bytes(),
        });

        let atoms = Atoms::intern_all(&conn).map_err(|err| WindowCreationError::XcbAtomInternFailed(err))?;

        conn.send_request(&x::ChangeProperty {
            mode: x::PropMode::Replace,
            window,
            property: atoms.wm_protocols,
            r#type: x::ATOM_ATOM,
            data: &[atoms.wm_del_window],
        });

        conn.send_request(&x::MapWindow { window });
    
        conn.flush().map_err(|err| WindowCreationError::XcbFlushFailed(err))?;

        Ok(Self {
            conn,
            window,
            atoms,
            scale_factor
        })
    }

    pub fn handle(&self) -> RawWindowHandle {
        let window = NonZeroU32::new(self.window.resource_id()).unwrap();
        RawWindowHandle::Xcb(XcbWindowHandle::new(window))
    }

    pub fn event_loop(&self, func: impl Fn(&mut EventInfo)) -> Result<(), EventLoopError> {
        loop {
            let event = self.conn.wait_for_event().map_err(|err| EventLoopError::XcbWaitForEventFailed(err))?;

            let event = match event {
                xcb::Event::X(x::Event::KeyPress(event)) => Some(Event::KeyPressed(event.detail() as u16)),
                xcb::Event::X(x::Event::KeyRelease(event)) => Some(Event::KeyReleased(event.detail() as u16)),
                
                xcb::Event::X(x::Event::EnterNotify(_)) => Some(Event::MouseEntered),
                xcb::Event::X(x::Event::LeaveNotify(_)) => Some(Event::MouseLeft),
                xcb::Event::X(x::Event::MotionNotify(event)) => {
                    let pos = PhysicalPosition {
                        x: event.event_x() as u32,
                        y: event.event_y() as u32
                    };
                    
                    Some(Event::MouseMoved(pos.to_logical(self.scale_factor)))
                },
                xcb::Event::X(x::Event::ButtonPress(event)) => Some(Event::MouseButtonPressed(map_mouse_button(event.detail()))),
                xcb::Event::X(x::Event::ButtonRelease(event)) => Some(Event::MouseButtonReleased(map_mouse_button(event.detail()))),
                xcb::Event::X(x::Event::ResizeRequest(event)) => {
                    let size = PhysicalSize {
                        width: event.width() as u32,
                        height: event.height() as u32
                    };

                    Some(Event::Resized(size.to_logical(self.scale_factor)))
                },
                xcb::Event::X(x::Event::ClientMessage(event)) => {
                    if let x::ClientMessageData::Data32([atom, ..]) = event.data() {
                        if atom == self.atoms.wm_del_window.resource_id() {
                            Some(Event::ShouldClose)
                        }
                        else {
                            None
                        }
                    }
                    else {
                        None
                    }
                },

                _ => None
            };

            if let Some(event) = event {
                let mut info = EventInfo {
                    event,
                    scale_factor: self.scale_factor,
                    exit: false
                };
                
                func(&mut info);

                if info.exit {
                    return Ok(());
                }
            }
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.conn.send_request(&x::DestroyWindow { window: self.window });
    }
}

fn map_mouse_button(button: xcb::x::Button) -> MouseButton {
    match button {
        1 => MouseButton::Left,
        2 => MouseButton::Middle,
        3 => MouseButton::Right,
        other => MouseButton::Other(other as u8)
    }
}