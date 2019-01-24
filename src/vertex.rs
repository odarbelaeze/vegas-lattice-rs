extern crate serde_json;

use std::str::FromStr;

use serde_json::Error as SerdeError;
use super::util::{Axis, Tagged, python_mod};


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
    fn tags<'a>(&'a self) -> Option<&'a Vec<String>> {
        match self.tags {
            Some(ref tags) => Some(&tags),
            None => None
        }
    }
}


impl Vertex {

    pub fn source(&self) -> usize {
        self.source
    }

    pub fn target(&self) -> usize {
        self.target
    }

    pub fn delta_along(&self, axis: Axis) -> i32 {
        match axis {
            Axis::X => self.delta.0,
            Axis::Y => self.delta.1,
            Axis::Z => self.delta.2,
        }
    }

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
    fn vertex_will_take_optional_tags() {
        let data = r#"
            {"source": 0, "target": 0, "delta": [0, 0, 1], "tags": ["core", "inner"]}
        "#;
        let vertex_result: Result<Vertex, _> = data.parse();
        assert!(vertex_result.is_ok());
        assert_eq!(vertex_result.unwrap().tags,
                   Some(vec!["core".to_string(), "inner".to_string()]));
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
