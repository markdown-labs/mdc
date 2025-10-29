//! Parser for markdown document with extension syntax.
//!

#![cfg_attr(docsrs, feature(doc_cfg))]

mod errors;
pub use errors::*;

mod input;
pub use input::*;

mod tokens;
pub use tokens::*;

mod block;
pub use block::*;
