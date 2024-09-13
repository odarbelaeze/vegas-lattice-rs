use crate::util::{python_mod, Axis, Tagged};
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeError;
use std::str::FromStr;

/// Represents a vertex in a lattice.
///
/// The `source` and `target` fields are the indices of the sites that the vertex connects.
/// The `delta` field is a tuple of the displacements of the target site from the source site.
/// A `delta` of `(0, 0, 1)` would mean that the target site is one unit along the z axis from the
/// source site.
///
/// For a simple cubic lattice the `source` and `target` for all the vertices would be the same,
/// `0`. While delta would be `(1, 0, 0)`, `(0, 1, 0)`, `(0, 0, 1)`. Those 3 vertices would connect
/// all the sites in the lattice.
///
/// # Examples
///
/// Here is an example of how to create a vertex and access its fields:
///
/// ```rust
/// use vegas_lattice::Vertex;
///
/// let vertex = Vertex::new(0, 1, (0, 0, 1));
/// assert_eq!(vertex.source(), 0);
/// assert_eq!(vertex.target(), 1);
/// assert_eq!(vertex.delta(), (0, 0, 1));
/// ```
///
/// Here's how to move a vertex along an axis:
///
/// ```rust
/// use vegas_lattice::Vertex;
///
/// let vertex = Vertex::new(0, 1, (0, 0, 1)).move_z(1, 2, 3);
/// assert_eq!(vertex.source(), 2);
/// assert_eq!(vertex.target(), 5);
/// assert_eq!(vertex.delta(), (0, 0, 0));
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vertex {
    source: usize,
    target: usize,
    delta: (i32, i32, i32),
    tags: Option<Vec<String>>,
}

impl FromStr for Vertex {
    type Err = SerdeError;
    fn from_str(source: &str) -> Result<Vertex, Self::Err> {
        serde_json::from_str(source)
    }
}

impl Tagged for Vertex {
    fn tags(&self) -> Option<&Vec<String>> {
        self.tags.as_ref()
    }
}

impl Vertex {
    /// Creates a new vertex with the given source and target
    pub fn new(source: usize, target: usize, delta: (i32, i32, i32)) -> Self {
        Vertex {
            source,
            target,
            delta,
            tags: None,
        }
    }

    /// Returns the `source` of the vertex
    pub fn source(&self) -> usize {
        self.source
    }

    /// Returns the `target` of the vertex
    pub fn target(&self) -> usize {
        self.target
    }

    /// Returns the `delta` of the vertex
    pub fn delta(&self) -> (i32, i32, i32) {
        self.delta
    }

    /// Chagges the tags of the vertex
    pub fn with_tags(mut self, tags: Vec<&str>) -> Self {
        self.tags = Some(tags.iter().map(|s| s.to_string()).collect());
        self
    }

    #[inline]
    fn delta_along(&self, axis: Axis) -> i32 {
        match axis {
            Axis::X => self.delta.0,
            Axis::Y => self.delta.1,
            Axis::Z => self.delta.2,
        }
    }

    /// Move the vertex along a given axis by a given amount
    ///
    /// The only reason you would want to move a vertex along an axis is to grow a lattice along
    /// that axis. That's why we're going to need to know the parameters of how much the lattice is
    /// going to grow by. This function takes in the `nsites`, the number of sites in the original
    /// lattice, and the `limit`, the total number of units the lattice will be grown by. The
    /// vertex will be changed assuming that the new number of sites will be `limit * nsites`.
    ///
    /// # Warning
    ///
    /// This function is so complicated it should be *private*!
    ///
    /// # Arguments
    ///
    /// * `axis` - The axis along which to move the vertex
    /// * `amount` - The number of units to move the vertex
    /// * `nsites` - The number of sites in the original lattice
    /// * `limit` - The total number of units the lattice will be grown by
    fn move_along(mut self, axis: Axis, amount: usize, nsites: usize, limit: usize) -> Self {
        debug_assert!(amount < limit);
        let distance = amount * nsites;
        let new_nsites = limit * nsites;
        self.source += distance;
        self.target += distance;
        let delta = self.delta_along(axis);
        let target = self.target as i32 + delta * nsites as i32;
        let (target, delta) = python_mod(target, new_nsites);
        self.target = target as usize;
        match axis {
            Axis::X => self.delta.0 = delta,
            Axis::Y => self.delta.1 = delta,
            Axis::Z => self.delta.2 = delta,
        };
        self
    }

    /// Move along the x axis
    pub fn move_x(self, amount: usize, nsites: usize, limit: usize) -> Self {
        self.move_along(Axis::X, amount, nsites, limit)
    }

    /// Move along the y axis
    pub fn move_y(self, amount: usize, nsites: usize, limit: usize) -> Self {
        self.move_along(Axis::Y, amount, nsites, limit)
    }

    /// Move along the z axis
    pub fn move_z(self, amount: usize, nsites: usize, limit: usize) -> Self {
        self.move_along(Axis::Z, amount, nsites, limit)
    }

    /// Re-index the vertex
    pub fn reindex(mut self, index: &[usize]) -> Self {
        self.source = index[self.source];
        self.target = index[self.target];
        self
    }
}

#[cfg(test)]
mod test {
    use super::Vertex;

    #[test]
    fn vertex_can_be_created() {
        let vertex = Vertex::new(0, 1, (0, 0, 1));
        assert_eq!(vertex.source, 0);
        assert_eq!(vertex.target, 1);
        assert_eq!(vertex.delta, (0, 0, 1));
    }

    #[test]
    fn vertex_can_be_moved() {
        // The vertex would point to the second site in the next unit along the z axis
        let vertex = Vertex::new(0, 1, (0, 0, 1)).move_along(super::Axis::Z, 0, 2, 2);
        assert_eq!(vertex.source, 0);
        assert_eq!(vertex.target, 3);
        assert_eq!(vertex.delta, (0, 0, 0));
    }

    #[test]
    fn vertex_can_be_moved_2() {
        // The vertex would point to the second site on the same unit along the z axis
        let vertex = Vertex::new(0, 1, (0, 0, 0)).move_along(super::Axis::Z, 0, 2, 2);
        assert_eq!(vertex.source, 0);
        assert_eq!(vertex.target, 1);
        assert_eq!(vertex.delta, (0, 0, 0));
    }

    #[test]
    fn vertex_can_be_moved_3() {
        // The vertex would lay on the second unit along the z axis
        let vertex = Vertex::new(0, 1, (0, 0, 0)).move_along(super::Axis::Z, 1, 2, 2);
        assert_eq!(vertex.source, 2);
        assert_eq!(vertex.target, 3);
        assert_eq!(vertex.delta, (0, 0, 0));
    }

    #[test]
    fn vertex_can_be_moved_4() {
        // The vertex would point to the second site on the first unit along the z axis but in the
        // next new delta.
        let vertex = Vertex::new(0, 1, (0, 0, 1)).move_along(super::Axis::Z, 1, 2, 2);
        assert_eq!(vertex.source, 2);
        assert_eq!(vertex.target, 1);
        assert_eq!(vertex.delta, (0, 0, 1));
    }

    #[test]
    fn vertex_can_be_tagged() {
        let vertex = Vertex::new(0, 1, (0, 0, 1)).with_tags(vec!["core"]);
        assert_eq!(vertex.tags, Some(vec!["core".to_string()]));
    }

    #[test]
    fn reindexing_kinda_works() {
        let vertex = Vertex::new(0, 1, (0, 0, 1)).reindex(&[1, 0]);
        assert_eq!(vertex.source, 1);
        assert_eq!(vertex.target, 0);
    }

    #[test]
    fn vertex_will_take_optional_tags() {
        let data = r#"
            {"source": 0, "target": 0, "delta": [0, 0, 1], "tags": ["core", "inner"]}
        "#;
        let vertex_result: Result<Vertex, _> = data.parse();
        assert!(vertex_result.is_ok());
        assert_eq!(
            vertex_result.unwrap().tags,
            Some(vec!["core".to_string(), "inner".to_string()])
        );
    }
}
