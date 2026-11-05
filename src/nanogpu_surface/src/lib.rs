#![no_std]

use nanogpu_render::RenderCommand;
use nanogpu_window::Window;

pub trait SurfaceBuilder<SurfaceType: Surface> {

    fn build(&self, window: impl Window) -> Result<SurfaceType, SurfaceBuildError>;
}

pub trait Surface {

    fn render(&mut self, commands: impl Iterator<Item = RenderCommand>);

    fn present(&mut self);
}

pub struct SurfaceBuildError {

    pub message: &'static str,
}

