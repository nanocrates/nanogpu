//! GLFW windowing backend for `nanogpu`.
//!
//! This backend provides [`GlfwWindow`], an implementation of [`Window`] with a GLFW backend.
//! All [`nanogpu_window`] features are supported.
//!
//! Platform compatibility:
//! - Windows: YES
//! - macOS: YES
//! - Linux: YES
//! - Web: NO
//! - Android: NO
//! - iOS: NO

mod key;

use glfw::{Context as _, GlfwReceiver};
use nanomath::vector::Vec2u;

use nanogpu_window::Window;
use nanogpu_window::error::WindowCreationError;
use nanogpu_window::event::{WindowEvent, KeyEvent};

/// An implementation of [`Window`] with a GLFW backend (desktop use).
///
/// See this [`crate`]'s documentiation for a full compatibility report.
pub struct GlfwWindow {

    /// The glfw library backend handle, used for various things, e.g. event polling.
    glfw_handle: glfw::Glfw,

    /// The stored handle to the glfw window itself.
    #[expect(clippy::struct_field_names, reason = "This is literally the *glfw window handle*")]
    glfw_window_handle: glfw::PWindow,

    /// The event pump for querying glfw messages. This is used by the implementation of
    /// [`Window::events`].
    glfw_event_pump: GlfwReceiver<(f64, glfw::WindowEvent)>,

    /// The size of the window, as a [`Vec2u`] of its width and height.
    /// API users can query this using [`Window::get_dimensions`].
    dimensions: Vec2u,

    /// The OS title of the window.
    /// API users can query this using [`Window::get_title`] and set it using
    /// [`Window::set_title`].
    title: String,

    /// Internal flag which is set to true only when a user manually calls the [`Window::close`]
    /// method.
    /// Used as *one of* the conditions for [`Window::is_open`].
    should_close: bool,
}

impl GlfwWindow {
    /// Create a new [`Window`] with a GLFW backend, using the provided dimensions (size) and window
    /// title.
    ///
    /// ## Parameters
    /// - `dimensions`: [`Vec2u`]: the size of the window (width and height).
    /// - `title`: [`&str`]: the titlebar name to give to the window.
    ///
    /// ## Errors
    /// A [`WindowCreationError`] is returned in one of two cases:
    /// - [`WindowCreationError::InitializeSubsystem`] if the GLFW backend itself failed to load.
    ///   This is usually because of missing libraries on the target machine.
    /// - [`WindowCreationError::CreateWindow`] if GLFW initialized fine, but the window itself
    ///   couldn't be created for any reason.
    pub fn new(dimensions: Vec2u, title: &str) -> Result<Self, WindowCreationError> {
        use glfw::fail_on_errors;
        let mut glfw_handle: glfw::Glfw = glfw::init(fail_on_errors!())
            .map_err(|glfw_error| {
                let msg = match glfw_error {
                    glfw::InitError::AlreadyInitialized => "GLFW has already been initialized, so it cannot be initialized again",
                    glfw::InitError::Internal => "There was an internal error in GLFW while initializing the library",
                };
                WindowCreationError::InitializeSubsystem(msg)
            })?;

        glfw_handle.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

        let (mut glfw_window_handle, glfw_event_pump) = glfw_handle.create_window(dimensions.x, dimensions.y, title, glfw::WindowMode::Windowed)
            .ok_or(WindowCreationError::CreateWindow("Failed to create GLFW window"))?;

        glfw_window_handle.set_all_polling(true);

        Ok(Self {
            glfw_handle,
            glfw_window_handle,
            glfw_event_pump,
            dimensions,
            title: title.to_owned(),
            should_close: false,
        })
    }
}


impl Window for GlfwWindow {

    fn get_dimensions(&self) -> &Vec2u {
        &self.dimensions
    }

    fn get_title(&self) -> &str {
        &self.title
    }

    fn set_title(&mut self, title: &str) {
        title.clone_into(&mut self.title);
        self.glfw_window_handle.set_title(&self.title);
    }

    fn events(&mut self) -> impl Iterator<Item = WindowEvent> {                                                                                      
        self.glfw_handle.poll_events();                                                                                                     

        #[expect(clippy::wildcard_enum_match_arm, reason = "For now, the only GLFW events we care about are listed in the match. If GLFW adds more events in the future, we will ignore them until this function is updated - additions to GLFW must not break this function.")]
        glfw::flush_messages(&self.glfw_event_pump)
            .filter_map(|(_, event)| match event {                                                               
                glfw::WindowEvent::Key(glfw_key, _, glfw_action, _) => convert_key_event(glfw_key, glfw_action),
                glfw::WindowEvent::Close => Some(WindowEvent::Close),                                                                                         
                _ => None,                                                                                                           
            })
    }

    fn is_open(&self) -> bool {
        !self.should_close && !self.glfw_window_handle.should_close()
    }

    fn close(&mut self) {
        self.glfw_window_handle.set_should_close(true);
    }

    fn new_frame(&mut self) {
        self.glfw_window_handle.swap_buffers();
    }
}

/// Convert the given [`glfw::Key`] (GLFW key code) and [`glfw::Action`] (key up/down/repeat event
/// type) to a Nanogpu [`InputEvent`].
///
/// Used only in [`GlfwWindow::events`] to reduce match arm complexity and improve readability.
const fn convert_key_event(glfw_key: glfw::Key, glfw_action: glfw::Action) -> Option<WindowEvent> {
    use self::key::glfw_key_to_key;

    let Some(keycode) = glfw_key_to_key(glfw_key) else {
        return None; // We can't use the ? operator in a const fn.
    };
    let key_event = KeyEvent { key: keycode };
    match glfw_action {
        glfw::Action::Press => Some(WindowEvent::KeyPress(key_event)),
        glfw::Action::Release => Some(WindowEvent::KeyRelease(key_event)),
        glfw::Action::Repeat => Some(WindowEvent::KeyRepeat(key_event)),
    }
}

