use std::str::FromStr;

use super::util::{python_mod, Axis, Tagged};
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeError;

/// Represents a vertex in a lattice
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
    /// Returns the `source` of the vertex
    pub fn source(&self) -> usize {
        self.source
    }

    /// Returns the `target` of the vertex
    pub fn target(&self) -> usize {
        self.target
    }

    /// Returns the `delta` of the vertex alogn a given axis
    pub fn delta_along(&self, axis: Axis) -> i32 {
        match axis {
            Axis::X => self.delta.0,
            Axis::Y => self.delta.1,
            Axis::Z => self.delta.2,
        }
    }

    /// Returns a new vertex moved a given distance along a given axis
    pub fn move_along(&self, axis: Axis, index: usize, nsites: usize, limit: usize) -> Self {
        let mut vertex = self.clone();
        let distance = index * nsites;
        let new_nsites = limit * nsites;
        vertex.source += distance;
        vertex.target += distance;
        let delta = vertex.delta_along(axis);
        let target = vertex.target as i32 + delta * nsites as i32;
        let (target, delta) = python_mod(target, new_nsites);
        vertex.target = target as usize;
        match axis {
            Axis::X => vertex.delta.0 = delta,
            Axis::Y => vertex.delta.1 = delta,
            Axis::Z => vertex.delta.2 = delta,
        };
        vertex
    }

    /// Returns a new vertex with the same properties but a re-indexed source and target
    pub fn reindex(&self, index: &[usize]) -> Self {
        let mut vertex = self.clone();
        vertex.source = index[self.source];
        vertex.target = index[self.target];
        vertex
    }
}

#[cfg(test)]
mod test {
    use super::Vertex;

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

    #[test]
    fn reindexing_kinda_works() {
        let data = r#"
            {"source": 0, "target": 1, "delta": [0, 0, 1], "tags": ["core", "inner"]}
        "#;
        let vertex: Vertex = data.parse().unwrap();
        let vertex = vertex.reindex(&[1, 0]);
        assert_eq!(vertex.source, 1);
        assert_eq!(vertex.target, 0);
    }
}
