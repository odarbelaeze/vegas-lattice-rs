//! # Intro
//!
//! This is a library for generating and manipulating 3D lattices.
//!

extern crate rand;
extern crate serde;
extern crate serde_json;

pub mod error;
pub mod io;

mod alloy;
mod lattice;
mod mask;
mod site;
mod util;
mod vertex;

pub use alloy::Alloy;
pub use lattice::Lattice;
pub use mask::Mask;
pub use site::Site;
pub use util::{Axis, Tagged};
pub use vertex::Vertex;
