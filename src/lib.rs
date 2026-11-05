//! A convenient abstraction over various windowing systems and graphics pipelines.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "math")]
pub use nanomath as math;

#[cfg(feature = "window")]
pub use nanogpu_window as window;

#[cfg(feature = "surface")]
pub use nanogpu_surface as surface;

#[cfg(feature = "glfw")]
pub use nanogpu_glfw as glfw;

#[cfg(feature = "wgpu")]
pub use nanogpu_wgpu as wgpu;
