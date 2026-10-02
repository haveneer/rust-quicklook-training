//! The [`Mesh`] type and its error type [`MeshError`].

use crate::geometry::triangle_area;
use std::fmt;

/// A 2D polygonal mesh, stored as an ordered list of nodes.
///
/// Build it with [`Mesh::new`], [`Mesh::with_capacity`] or [`Mesh::from_points`].
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Mesh {
    nodes: Vec<[f64; 2]>, // private: invariants stay under the crate's control
}

impl Mesh {
    /// An empty mesh.
    pub fn new() -> Self {
        Self::default()
    }

    /// An empty mesh with room for `n` nodes without reallocating.
    pub fn with_capacity(n: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(n),
        }
    }

    /// A mesh from its nodes, in order.
    pub fn from_points(nodes: Vec<[f64; 2]>) -> Self {
        Self { nodes }
    }

    /// Number of nodes.
    #[doc(alias = "len")]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// `true` if the mesh has no node.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// The nodes, borrowed (a getter: no `get_` prefix).
    pub fn nodes(&self) -> &[[f64; 2]] {
        &self.nodes
    }

    /// The nodes, mutably borrowed.
    pub fn nodes_mut(&mut self) -> &mut [[f64; 2]] {
        &mut self.nodes
    }

    /// Consumes the mesh and gives its nodes back, without copying.
    pub fn into_nodes(self) -> Vec<[f64; 2]> {
        self.nodes
    }

    /// Area enclosed by the nodes, taken as a polygon.
    ///
    /// # Errors
    ///
    /// Returns [`MeshError::TooFewNodes`] if the mesh has fewer than 3 nodes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use docdemo::{Mesh, MeshError};
    /// # fn main() -> Result<(), MeshError> {
    /// let square = Mesh::from_points(vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]);
    /// assert_eq!(square.polygon_area()?, 1.0);
    /// # Ok(())
    /// # }
    /// ```
    pub fn polygon_area(&self) -> Result<f64, MeshError> {
        let [first, rest @ ..] = self.nodes.as_slice() else {
            return Err(MeshError::TooFewNodes { found: 0 });
        };
        if rest.len() < 2 {
            return Err(MeshError::TooFewNodes {
                found: self.nodes.len(),
            });
        }
        Ok(rest
            .windows(2)
            .map(|w| triangle_area(*first, w[0], w[1]))
            .sum::<f64>()
            .abs())
    }

    /// Multiplies every coordinate by `factor`.
    ///
    /// # Panics
    ///
    /// Panics if `factor` is not strictly positive and finite.
    ///
    /// ```should_panic
    /// docdemo::Mesh::new().scale(0.0);
    /// ```
    pub fn scale(&mut self, factor: f64) {
        assert!(
            factor.is_finite() && factor > 0.0,
            "invalid scale factor {factor}"
        );
        self.nodes.iter_mut().flatten().for_each(|x| *x *= factor);
    }
}

/// Errors returned by [`Mesh`] operations.
///
/// The nodes stay private, so this does not compile:
///
/// ```compile_fail
/// let mesh = docdemo::Mesh::new();
/// let n = mesh.nodes.len(); // error[E0616]: field `nodes` is private
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MeshError {
    /// The operation needs at least 3 nodes.
    TooFewNodes {
        /// How many nodes the mesh has.
        found: usize,
    },
}

impl fmt::Display for MeshError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooFewNodes { found } => write!(f, "need at least 3 nodes, found {found}"),
        }
    }
}

impl std::error::Error for MeshError {}
