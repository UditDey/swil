//! UI scaling/DPI aware geometry types
//! 
//! Swil uses a DPI handling system similar to `winit`. Every window has an associated
//! 'scale factor' which can be used to create consistently sized UIs on screens with
//! different DPIs. The scale factor respects OS wide UI scale settings
//! 
//! The [`PhysicalPosition`]/[`PhysicalSize`] types represent physical pixels on the
//! screen. The ['LogicalPosition']/['LogicalSize'] types correspond to physical
//! pixel units divided by the scale factor. By using the logical types when creating
//! UI elements, you can ensure that the UI is consistently sized on different screens
//! 
//! # Scale Factor Calculation
//! Swil tries to calculate the scale factor similarly to `winit`
//! - **X11**: 
//!   + Swil tries to read `Xft.dpi` from the `Xresources` resource database. The scale
//!     factor will be `Xft.dpi / 96`
//!   + If `Xft.dpi` is not present, calculate the DPI using the screen dimensions
//!     reported by the X server

/// A position that is either physical or logical
#[derive(Debug, Clone)]
pub enum Position {
    Physical(PhysicalPosition),
    Logical(LogicalPosition)
}

/// A size that is either physical or logical
#[derive(Debug, Clone)]
pub enum Size {
    Physical(PhysicalSize),
    Logical(LogicalSize)
}

/// A position in physical pixel units
#[derive(Debug, Clone)]
pub struct PhysicalPosition {
    pub x: u32,
    pub y: u32
}

impl PhysicalPosition {
    pub fn from_logical(position: LogicalPosition, scale_factor: f32) -> Self {
        let x = (position.x * scale_factor).round() as u32;
        let y = (position.y * scale_factor).round() as u32;

        Self { x, y }
    }

    pub fn to_logical(&self, scale_factor: f32) -> LogicalPosition {
        let x = self.x as f32 / scale_factor;
        let y = self.y as f32 / scale_factor;

        LogicalPosition { x, y }
    }
}

/// A size in physical pixel units
#[derive(Debug, Clone)]
pub struct PhysicalSize {
    pub width: u32,
    pub height: u32
}

impl PhysicalSize {
    pub fn from_logical(size: LogicalSize, scale_factor: f32) -> Self {
        let width = (size.width * scale_factor).round() as u32;
        let height = (size.height * scale_factor).round() as u32;

        Self { width, height }
    }

    pub fn to_logical(&self, scale_factor: f32) -> LogicalSize {
        let width = self.width as f32 / scale_factor;
        let height = self.height as f32 / scale_factor;

        LogicalSize { width, height }
    }
}

/// A position in scaled logical units
#[derive(Debug, Clone)]
pub struct LogicalPosition {
    pub x: f32,
    pub y: f32
}

impl LogicalPosition {
    pub fn from_physical(position: PhysicalPosition, scale_factor: f32) -> Self {
        position.to_logical(scale_factor)
    }

    pub fn to_physical(&self, scale_factor: f32) -> PhysicalPosition {
        PhysicalPosition::from_logical(self.clone(), scale_factor)
    }
}

/// A size in scaled logical units
#[derive(Debug, Clone)]
pub struct LogicalSize {
    pub width: f32,
    pub height: f32
}

impl LogicalSize {
    pub fn from_physical(size: PhysicalSize, scale_factor: f32) -> Self {
        size.to_logical(scale_factor)
    }

    pub fn to_physical(&self, scale_factor: f32) -> PhysicalSize {
        PhysicalSize::from_logical(self.clone(), scale_factor)
    }
}