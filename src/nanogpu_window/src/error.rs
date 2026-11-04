use core::fmt;

use fmt::Display;

use nanomath::vector::Vec2u;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowCreationError {
    InitializeSubsystem(&'static str),
    CreateWindow(&'static str),
    InvalidDimensions(Vec2u),
    InvalidTitle(&'static str),
}

impl Display for WindowCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InitializeSubsystem(msg) => write!(f, "Failed to initialize window subsystem: {msg}"),
            Self::CreateWindow(msg) => write!(f, "Failed to create window: {msg}"),
            Self::InvalidDimensions(dimensions) => write!(f, "Failed to create window due to invalid window dimensions: {dimensions}"),
            Self::InvalidTitle(title) => write!(f, "Failed to create window due to invalid title: {title}"),
        }
    }
}

