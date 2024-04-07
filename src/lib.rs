//! Formatic
//! A libary to create object files

pub mod decl;
pub mod error;
pub mod link;
pub mod object_builder;

pub use decl::*;
pub use error::*;
pub use link::*;
pub use object_builder::*;
