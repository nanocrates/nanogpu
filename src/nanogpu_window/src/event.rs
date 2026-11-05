//! Types for runtime events with the nanogpu window. The primary type is the [`WindowEvent`] enum.
use super::key::Key;

/// Represents a generic event that occured to a window. You can loop over these and `match` for
/// specific events.
///
/// See the individual variants for more information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowEvent {

    /// The window has been requested to close. You should respond to this by cleaning up any
    /// resources, saving any state, or any other shutdown logic.
    ///
    /// Whether or not the window itself will be closed automatically or you need to manually call
    /// `window.close()` is backend-dependent - but it is always acceptable and memory safe to
    /// manually close it, even after it has already closed.
    Close,

    /// A key on the keyboard was pressed down for the first time.
    ///
    /// Two consecutive [`WindowEvent::KeyPress`] events will always be separated by a
    /// [`WindowEvent::KeyRelease`]; a user cannot press a key down twice if they don't release it
    /// in-between.
    ///
    /// If you want to handle key *repeats*, please match for both this and [`WindowEvent::KeyRepeat`].
    KeyPress(KeyEvent),

    /// A key on the keyboard was released after already being pressed down.
    ///
    /// Two consecutive [`WindowEvent::KeyRelease`] events will always be separated by a
    /// [`WindowEvent::KeyPress`]; a user cannot release a key twice if they don't press it down
    /// in-between.
    KeyRelease(KeyEvent),

    /// A key on the keyboard was repeated. This means that it has already been pressed earlier, but
    /// not released, so the operating system treats it as a key repeat.
    ///
    /// This event will only ever *succeed* [`WindowEvent::KeyPress`].
    KeyRepeat(KeyEvent),
}

/// Structure for the parameters to a generic key-related event.
/// See the fields of this struct for more information about what is stored.
///
/// The type of action will be given in context from [`WindowEvent`]; this stores only the body data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyEvent {

    /// The [`Key`] in question that was pressed/released/repeated.
    pub key: Key,
}

