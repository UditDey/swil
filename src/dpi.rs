#[derive(Debug, Clone)]
pub enum EitherPosition {
    Logical(LogicalPosition),
    Physical(PhysicalPosition)
}

impl EitherPosition {
    pub fn to_physical(&self, scale_factor: f32) -> PhysicalPosition {
        match self {
            Self::Logical(size) => size.to_physical(scale_factor),
            Self::Physical(size) => size.clone()
        }
    }
}

impl From<(f32, f32)> for EitherPosition {
    fn from((x, y): (f32, f32)) -> Self {
        Self::Logical(LogicalPosition { x, y })
    }
}

#[derive(Debug, Clone)]
pub struct LogicalPosition {
    pub x: f32,
    pub y: f32
}

impl LogicalPosition {
    pub fn to_physical(&self, scale_factor: f32) -> PhysicalPosition {
        PhysicalPosition {
            x: (self.x * scale_factor) as u32,
            y: (self.y * scale_factor) as u32
        }
    }
}

impl From<(f32, f32)> for LogicalPosition {
    fn from((x, y): (f32, f32)) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
pub struct PhysicalPosition {
    pub x: u32,
    pub y: u32
}

impl PhysicalPosition {
    pub fn to_logical(&self, scale_factor: f32) -> LogicalPosition {
        LogicalPosition {
            x: self.x as f32 / scale_factor,
            y: self.y as f32 / scale_factor
        }
    }
}

impl From<(u32, u32)> for PhysicalPosition {
    fn from((x, y): (u32, u32)) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
pub enum EitherSize {
    Logical(LogicalSize),
    Physical(PhysicalSize)
}

impl EitherSize {
    pub fn to_physical(&self, scale_factor: f32) -> PhysicalSize {
        match self {
            Self::Logical(size) => size.to_physical(scale_factor),
            Self::Physical(size) => size.clone()
        }
    }
}

impl From<(f32, f32)> for EitherSize {
    fn from((width, height): (f32, f32)) -> Self {
        Self::Logical(LogicalSize { width, height })
    }
}

#[derive(Debug, Clone)]
pub struct LogicalSize {
    pub width: f32,
    pub height: f32
}

impl LogicalSize {
    pub fn to_physical(&self, scale_factor: f32) -> PhysicalSize {
        PhysicalSize {
            width: (self.width * scale_factor) as u32,
            height: (self.height * scale_factor) as u32
        }
    }
}

impl From<(f32, f32)> for LogicalSize {
    fn from((width, height): (f32, f32)) -> Self {
        Self { width, height }
    }
}

#[derive(Debug, Clone)]
pub struct PhysicalSize {
    pub width: u32,
    pub height: u32
}

impl PhysicalSize {
    pub fn to_logical(&self, scale_factor: f32) -> LogicalSize {
        LogicalSize {
            width: self.width as f32 / scale_factor,
            height: self.height as f32 / scale_factor
        }
    }
}

impl From<(u32, u32)> for PhysicalSize {
    fn from((width, height): (u32, u32)) -> Self {
        Self { width, height }
    }
}