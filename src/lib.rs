//! Load and render XPS documents

#![warn(
    missing_docs,
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms,
    unused
)]

mod archive;
mod error;
mod parts;
mod relationships;
mod renderer;
mod xps;

pub use crate::renderer::Renderer;
pub use crate::xps::XPS;

#[cfg(feature = "cairo-renderer")]
mod cairo_renderer;

#[cfg(feature = "cairo-renderer")]
pub use crate::cairo_renderer::CairoRenderer;
