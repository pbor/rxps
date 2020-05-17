//! Load and render XPS documents

#![warn(
    missing_docs,
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms,
    unused
)]

mod archive;
mod document;
mod error;
mod page;
mod parts;
mod relationships;
mod xps;

pub use crate::document::{Document, Outline};
pub use crate::page::Page;
pub use crate::xps::XPS;
