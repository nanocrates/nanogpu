use glfw::{Context as _, GlfwReceiver};
use nanomath::vector::Vec2u;

use crate::window::Window;
use crate::window::error::WindowCreationError;
use crate::window::event::{WindowEvent, KeyEvent};

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
        let mut glfw = glfw::init(fail_on_errors!())
            .map_err(|glfw_error| WindowCreationError::InitializeSubsystem(glfw_error.to_string()))?;

        let (mut glfw_window, glfw_events) = glfw.create_window(dimensions.x, dimensions.y, title, glfw::WindowMode::Windowed)
            .ok_or_else(|| WindowCreationError::CreateWindow("Failed to create GLFW window".to_owned()))?;

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

    fn events(&mut self) -> Vec<WindowEvent> {
        let mut events = Vec::new();
        self.glfw.poll_events();
        let glfw_events: Vec<(f64, glfw::WindowEvent)> = glfw::flush_messages(&self.glfw_events).collect();
        for (_, glfw_event) in glfw_events {
            #[expect(clippy::wildcard_enum_match_arm, reason = "For now, the only GLFW events we care about are listed in the match. If GLFW adds more events in the future, we will ignore them until this function is updated - additions to GLFW must not break this function.")]
            if let Some(input_event) = match glfw_event {
                glfw::WindowEvent::Key(key, _scancode, action, _mods) => convert_key_event(key, action),
                glfw::WindowEvent::Close => {
                    self.close();
                    Some(WindowEvent::Close)
                },
                _ => None,
            } {
                events.push(input_event);
            }
        }
        events
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
    use crate::window::event::WindowEvent;
    use crate::window::key::Key;

    #[expect(clippy::as_conversions, reason = "GLFW key codes are enum variants with i32 values. We can't use `TryFrom` because glfw::Key only *represents* an i32, but doesn't actually implement `TryFrom<i32>`. `as u32` is safe here because all GLFW key codes are non-negative and fit in a u32.")]
    let keycode = Key::from_u32(glfw_key as u32)?;

    let key_event = KeyEvent { key: keycode };
    match glfw_action {
        glfw::Action::Press => Some(WindowEvent::KeyPress(key_event)),
        glfw::Action::Release => Some(WindowEvent::KeyRelease(key_event)),
        glfw::Action::Repeat => Some(WindowEvent::KeyRepeat(key_event)),
    }
}

