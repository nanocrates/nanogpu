#![no_std]

pub mod event;
pub mod key;
pub mod error;

use nanomath::vector::Vec2u;

use self::event::WindowEvent;

pub trait Window: Sized {

    fn get_dimensions(&self) -> &Vec2u;

    fn get_title(&self) -> &str;

    fn set_title(&mut self, title: &str);

    fn events(&mut self) -> impl Iterator<Item = WindowEvent>;

    fn is_open(&self) -> bool;

    fn close(&mut self);
    
    fn redraw(&mut self);

    fn get_width(&self) -> u32 {
        self.get_dimensions().x
    }

    fn get_height(&self) -> u32 {
        self.get_dimensions().y
    }
}

