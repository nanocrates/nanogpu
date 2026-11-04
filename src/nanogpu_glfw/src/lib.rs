mod key;

use glfw::{Context as _, GlfwReceiver};
use nanomath::vector::Vec2u;

use nanogpu_window::Window;
use nanogpu_window::error::WindowCreationError;
use nanogpu_window::event::{WindowEvent, KeyEvent};

pub struct GlfwWindow {
    glfw: glfw::Glfw,
    glfw_window: glfw::PWindow,
    glfw_events: GlfwReceiver<(f64, glfw::WindowEvent)>,
    dimensions: Vec2u,
    title: String,
    should_close: bool,
}

impl GlfwWindow {
    pub fn new(dimensions: Vec2u, title: &str) -> Result<Self, WindowCreationError> {
        use glfw::fail_on_errors;
        let mut glfw: glfw::Glfw = glfw::init(fail_on_errors!())
            .map_err(|glfw_error| {
                let msg = match glfw_error {
                    glfw::InitError::AlreadyInitialized => "GLFW has already been initialized, so it cannot be initialized again",
                    glfw::InitError::Internal => "There was an internal error in GLFW while initializing the library",
                };
                WindowCreationError::InitializeSubsystem(msg)
            })?;

        let (mut glfw_window, glfw_events) = glfw.create_window(dimensions.x, dimensions.y, title, glfw::WindowMode::Windowed)
            .ok_or(WindowCreationError::CreateWindow("Failed to create GLFW window"))?;

        glfw_window.make_current();
        glfw_window.set_all_polling(true);

        Ok(Self {
            glfw,
            glfw_window,
            glfw_events,
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
        self.glfw_window.set_title(&self.title);
    }

    fn events(&mut self) -> impl Iterator<Item = WindowEvent> {                                                                                      
        self.glfw.poll_events();                                                                                                     

        #[expect(clippy::wildcard_enum_match_arm, reason = "For now, the only GLFW events we care about are listed in the match. If GLFW adds more events in the future, we will ignore them until this function is updated - additions to GLFW must not break this function.")]
        glfw::flush_messages(&self.glfw_events)
            .filter_map(|(_, event)| match event {                                                               
                glfw::WindowEvent::Key(glfw_key, _, glfw_action, _) => convert_key_event(glfw_key, glfw_action),
                glfw::WindowEvent::Close => Some(WindowEvent::Close),                                                                                         
                _ => None,                                                                                                           
            })
    }

    fn is_open(&self) -> bool {
        !self.should_close && !self.glfw_window.should_close()
    }

    fn close(&mut self) {
        self.glfw_window.set_should_close(true);
    }

    fn redraw(&mut self) {
        self.glfw_window.swap_buffers();
    }
}

/// Convert the given [`glfw::Key`] (GLFW key code) and [`glfw::Action`] (key up/down/repeat event
/// type) to a Nanogpu [`InputEvent`].
///
/// Used only in [`GlfwWindow::events`] to reduce match arm complexity and improve readability.
fn convert_key_event(glfw_key: glfw::Key, glfw_action: glfw::Action) -> Option<WindowEvent> {
    use self::key::glfw_key_to_key;

    let keycode = glfw_key_to_key(glfw_key)?;
    let key_event = KeyEvent { key: keycode };
    match glfw_action {
        glfw::Action::Press => Some(WindowEvent::KeyPress(key_event)),
        glfw::Action::Release => Some(WindowEvent::KeyRelease(key_event)),
        glfw::Action::Repeat => Some(WindowEvent::KeyRepeat(key_event)),
    }
}

