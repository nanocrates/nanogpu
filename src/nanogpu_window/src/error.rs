//! Types for windowing-related errors in nanogpu.

use core::fmt;

use fmt::Display;

use nanomath::vector::Vec2u;

/// Error type indicating that something went wrong while creating the window.
/// This is returned from window creation functions.
///
/// If you get this error, you won't be able to continue with any graphics in your program, at
/// least, not for that window backend. Perhaps you want to try initializing a different windowing
/// backend if the primary fails?
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowCreationError {

    /// There was a problem when initializing the backend for the windowing library. For example,
    /// this could indicate that a required system library was not installed on the user's machine.
    InitializeSubsystem(&'static str),

    /// There was a problem while creating the window itself, but the library probably initialised
    /// fine. This likely indicates incorrect parameters or a headless user environment.
    CreateWindow(&'static str),

    /// The dimension [`Vec2u`] provided to the window creation / resize function was invalid.
    /// Usually, this means that the window size requested was either too big or too small.
    InvalidDimensions(Vec2u),

    /// The title given to the window creation / `set_title` function was invalid.
    /// Perhaps it contained invalid character codes not supported on the target platform?
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

