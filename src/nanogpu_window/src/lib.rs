//! `nanogpu_window` is a type definition crate which defines an interface for all `nanogpu`
//! **window** backends to implement.
//!
//! All windowing libraries compatible with the rest of `nanogpu` (such as the graphics pipeline)
//! must implement the traits and use the structs provided in this package.
//!
//! The main trait is [`Window`], which all windowing backend implementations MUST implement.
//! There are also traits and structs for general windowing features, such as [`event`]s.

#![no_std]

pub mod event;
pub mod key;
pub mod error;

use nanomath::vector::Vec2u;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use self::event::WindowEvent;

/// The main trait defining a window in nanogpu.
///
/// You should implement this trait on any window struct in a windowing backend.
/// You should *use* this trait in any graphics pipeline backends which require drawing to a window.
///
/// > This solves the NxM problem, as any graphics backend can use any windowing library, provided
/// > it implements this trait.
pub trait Window: Sized + HasWindowHandle + HasDisplayHandle {

    /// Get the dimensions (size) of the window, as a [`Vec2u`].
    fn get_dimensions(&self) -> &Vec2u;

    /// Get the title in the titlebar of the window, as a string slice.
    fn get_title(&self) -> &str;

    /// Change the current title of the window.
    fn set_title(&mut self, title: &str);

    /// Poll for all [`WindowEvent`]s.
    ///
    /// This returns all the windowing events that have occured since the last time this function
    /// was called. Loop through this to handle events.
    ///
    /// To make nanogpu `no_std`-compatible, this returns an iterator. You can use it like normal
    /// in a for loop.
    fn events(&mut self) -> impl Iterator<Item = WindowEvent>;

    /// Returns `true` if the window is still open and running, `false` if it is closed or scheduled
    /// to close.
    fn is_open(&self) -> bool;

    /// Force the window to close. The window UI itself will close, and any subsequent calls to
    /// [`Window::is_open`] will return `false`.
    fn close(&mut self);
    
    /// Redraw the window surface with the next frame.
    ///
    /// Because all draw calls have already happened to the buffer in earlier function calls, most
    /// window backends implement this as a simple buffer swap, meaning this function call is often
    /// very fast.
    fn new_frame(&mut self);

    /// Returns the width dimension of the window.
    /// This is the same as calling `Window::get_dimensions().x`; it returns the [`Vec2u::x`].
    fn get_width(&self) -> u32 {
        self.get_dimensions().x
    }

    /// Returns the height dimension of the window.
    /// This is the same as calling `Window::get_dimensions().y`; it returns the [`Vec2u::y`].
    fn get_height(&self) -> u32 {
        self.get_dimensions().y
    }
}

