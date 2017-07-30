extern crate serde_json;

use std::str::FromStr;

use serde_json::Error as SerdeError;
use super::util::{Axis, python_mod};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vertex {
    pub source: usize,
    pub target: usize,
    pub delta: (i32, i32, i32),
    pub tags: Option<Vec<String>>,
}


impl FromStr for Vertex {
    type Err = SerdeError;
    fn from_str(source: &str) -> Result<Vertex, Self::Err> {
        serde_json::from_str(source)
    }
}


impl Vertex {
    fn delta_along(&self, axis: Axis) -> i32 {
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
}