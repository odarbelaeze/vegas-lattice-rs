//! # Intro
//!
//! This is a library for generating and manipulating 3D lattices.
//!
//!
//! # Examples
//!
//! ## Lattice creation
//!
//! `vegas_lattice` provides simple ways to create cubic and body centered cubic lattices. But you
//! can create any number of custom lattices by creating the sites and edges manually.
//!
//! Here is an example of how to create a simple cubic lattice:
//!
//! ```rust
//! use vegas_lattice::Lattice;
//!
//! let lattice = Lattice::sc(1.0);
//!
//! assert_eq!(lattice.size(), (1.0, 1.0, 1.0));
//! assert_eq!(lattice.sites().len(), 1);
//! assert_eq!(lattice.edges().len(), 3);
//! ```
//!
//! Here is an example of how to create a body centered cubic lattice:
//!
//! ```rust
//! use vegas_lattice::Lattice;
//!
//! let lattice = Lattice::bcc(1.0);
//!
//! assert_eq!(lattice.size(), (1.0, 1.0, 1.0));
//! assert_eq!(lattice.sites().len(), 2);
//! assert_eq!(lattice.edges().len(), 8);
//! ```
//!
//! ## Lattice expansion
//!
//!
//! The most crucial operation to vegas lattice is expand, this is the one that takes a lattice
//! (usually a unitcell) and expands it to a bigger lattice, for example, if you take a cubic
//! lattice and apply the following operation:
//!
//! ```rust
//! use vegas_lattice::Lattice;
//!
//! let lattice = Lattice::sc(1.0).expand_x(2);
//!
//! assert_eq!(lattice.size(), (2.0, 1.0, 1.0));
//! assert_eq!(lattice.sites().len(), 2);
//! assert_eq!(lattice.edges().len(), 6);
//! ```
//!
//! You will end up with a lattice that is twice as big in the x direction. Notice how the number
//! of sites and edges is also doubled. This is because the lattice is expanded by replicating
//! the original lattice in the _x_ direction. The same operation can be applied to the _y_ and
//! _z_. The edges of the lattice are also replicated.
//!
//! ## Vertex dropping
//!
//! Well, as you saw in the expand operation, we keep the edges that have a delta those belong
//! to neighboring cells and can be interpreted as periodic boundary conditions, they also allow
//! for future expansions of the lattice, if you want to _drop_ this behavour you can use the
//! `drop` operation. Let's remove the periodic boundary conditions of the cubic lattice example we
//! did before:
//!
//! ```rust
//! use vegas_lattice::Lattice;
//!
//! let lattice = Lattice::sc(1.0).drop_x();
//! assert_eq!(lattice.size(), (1.0, 1.0, 1.0));
//! assert_eq!(lattice.sites().len(), 1);
//! assert_eq!(lattice.edges().len(), 2);
//! ```
//!
//! ## Masking
//!
//! The `apply_mask` operation allows you to remove sites from the lattice according to a mask. The
//! mask is a function that takes the coordinates of a site and returns a boolean. If the function
//! returns `true`, the site is kept, otherwise it is removed. This is useful to create composite
//! materials by drawing an image of them.
//!
//! ```rust
//! use vegas_lattice::{Lattice, Mask};
//! use std::path::Path;
//!
//! let mask = Mask::try_new(Path::new("docs/pattern.png"), 100.0).unwrap();
//! let mut rng = rand::rng();
//! let lattice = Lattice::sc(1.0)
//!     .expand_x(2)
//!     .expand_y(2)
//!     .expand_z(2)
//!     .apply_mask(mask, &mut rng);
//! assert_eq!(lattice.size(), (2.0, 2.0, 2.0));
//! assert!(lattice.sites().len() <= 8);
//! assert!(lattice.edges().len() <= 24);
//! ```
//!
//! ## Alloying
//!
//! The `alloy_sites` operation allows you to replace sites labeled as `source` with sites in the
//! `target` alloy. This is useful to create alloy materials by mixing different materials.
//!
//! ```rust
//! use vegas_lattice::{Alloy, Lattice};
//!
//! let alloy = Alloy::try_new(vec!["B", "C"], vec![50, 50]).unwrap();
//! let mut rng = rand::rng();
//! let lattice = Lattice::sc(1.0).alloy_sites("A", alloy, &mut rng);
//! assert_eq!(lattice.sites().len(), 1);
//! assert!(lattice.sites()[0].kind() == "B" || lattice.sites()[0].kind() == "C");
//! ```

pub mod error;
pub mod io;

mod alloy;
mod edge;
mod lattice;
mod mask;
mod site;
mod util;

pub use alloy::Alloy;
pub use edge::Edge;
pub use lattice::Lattice;
pub use mask::Mask;
pub use site::Site;
pub use util::Tagged;
