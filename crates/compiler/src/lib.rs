//! Parser for markdown document with extension syntax.
//!

#![cfg_attr(docsrs, feature(doc_cfg))]

mod errors;
pub use errors::*;

mod input;
pub use input::*;

mod item;

mod escaped;
pub use escaped::*;

mod entity;
pub use entity::*;

mod leaf;
pub use leaf::*;
