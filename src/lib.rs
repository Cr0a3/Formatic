//! Formatic
//! A libary to create object files

pub mod error;
pub mod object_builder;
pub mod decl;
pub mod link;

pub use error::*;
pub use object_builder::*;
pub use decl::*;
pub use link::*;