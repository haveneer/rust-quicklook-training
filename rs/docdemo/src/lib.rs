//! A tiny 2D mesh library, used to illustrate a crate's public API and its documentation.
//!
//! Most users only need the [`prelude`]:
//!
//! ```
//! use docdemo::prelude::*;
//!
//! let mesh = Mesh::from_points(vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]]);
//! assert_eq!(mesh.node_count(), 3);
//! ```

mod geometry; // private module: implementation detail
pub mod mesh;

pub use mesh::{Mesh, MeshError}; // re-exported: `docdemo::Mesh`, not `docdemo::mesh::Mesh`

/// The types needed by most users, in a single `use docdemo::prelude::*;`.
pub mod prelude {
    pub use crate::mesh::{Mesh, MeshError};
}
