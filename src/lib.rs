extern crate itertools;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::error::Error as StdError;
use std::fmt;

use itertools::Itertools;
use serde_json::Error as SerdeError;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Site {
    kind: String,
    position: (f64, f64, f64),
    tags: Option<Vec<String>>,
}


impl std::str::FromStr for Site {
    type Err = SerdeError;
    fn from_str(source: &str) -> Result<Site, Self::Err> {
        serde_json::from_str(source)
    }
}


impl Site {
    pub fn move_along(&self, axis: Axis, distance: f64) -> Self {
        let mut site = self.clone();
        match axis {
            Axis::X => site.position.0 += distance,
            Axis::Y => site.position.1 += distance,
            Axis::Z => site.position.2 += distance,
        };
        site
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Vertex {
    source: usize,
    target: usize,
    delta: (i32, i32, i32),
    tags: Option<Vec<String>>,
}


impl std::str::FromStr for Vertex {
    type Err = SerdeError;
    fn from_str(source: &str) -> Result<Vertex, Self::Err> {
        serde_json::from_str(source)
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Lattice {
    sites: Vec<Site>,
    vertices: Vec<Vertex>,
}


#[derive(Debug)]
pub enum LatticeError {
    JsonParseError(SerdeError),
    InconsistentVertices,
}


impl fmt::Display for LatticeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LatticeError::JsonParseError(_) => f.write_str("JsonParseError"),
            LatticeError::InconsistentVertices => f.write_str("InconsistentVertices"),
        }
    }
}


impl StdError for LatticeError {
    fn description(&self) -> &str {
        match *self {
            LatticeError::JsonParseError(_) => "Failed to parse JSON.",
            LatticeError::InconsistentVertices => "The vertices are inconsistent.",
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub enum Axis { X, Y, Z }


impl Lattice {
    fn are_vertices_consistent(&self) -> bool {
        self.vertices
            .iter()
            .map(|vertex| vertex.source)
            .chain(self.vertices.iter().map(|vertex| vertex.target))
            .all(|id| id < self.sites.len())
    }

    pub fn validate(self) -> Result<Self, LatticeError> {
        if !self.are_vertices_consistent() {
            return Err(LatticeError::InconsistentVertices);
        }
        Ok(self)
    }

    pub fn drop(mut self, axis: Axis) -> Self {
        self.vertices = match axis {
            Axis::X => self.vertices.into_iter().filter(|v| v.delta.0 == 0).collect(),
            Axis::Y => self.vertices.into_iter().filter(|v| v.delta.1 == 0).collect(),
            Axis::Z => self.vertices.into_iter().filter(|v| v.delta.2 == 0).collect(),
        };
        self
    }

    pub fn expand_along(mut self, axis: Axis, size: usize) -> Self {
        self.sites = (0..size)
            .cartesian_product(self.sites)
            // TODO this is going to break because of the size
            .map(|(index, site)| site.move_along(axis, (index * 1) as f64))
            .collect();
        self
    }
}


impl std::str::FromStr for Lattice {
    type Err = LatticeError;
    fn from_str(source: &str) -> Result<Lattice, Self::Err> {
        serde_json::from_str(source)
            .map_err(|err| LatticeError::JsonParseError(err))
            .and_then(|lattice: Lattice| lattice.validate())
    }
}


#[cfg(test)]
mod test {
    use super::{Site, Vertex, Lattice, Axis};

    #[test]
    fn site_will_take_optional_tags() {
        let data = r#"
            {"kind": "Fe", "position": [0, 0, 0], "tags": ["core", "inner"]}
        "#;
        let site_result: Result<Site, _> = data.parse();
        assert!(site_result.is_ok());
        assert_eq!(site_result.unwrap().tags,
                   Some(vec!["core".to_string(), "inner".to_string()]));
    }


    #[test]
    fn vertex_will_take_optional_tags() {
        let data = r#"
            {"source": 0, "target": 0, "delta": [0, 0, 1], "tags": ["core", "inner"]}
        "#;
        let site_result: Result<Vertex, _> = data.parse();
        assert!(site_result.is_ok());
        assert_eq!(site_result.unwrap().tags,
                   Some(vec!["core".to_string(), "inner".to_string()]));
    }

    #[test]
    fn drop_example() {
        let data = r#"
            {
                "sites": [
                    {"kind": "Fe", "position": [0, 0, 0]}
                ],
                "vertices": [
                    {"source": 0, "target": 0, "delta": [0, 0, 1], "tags": ["core", "inner"]}
                ]
            }
        "#;
        let lattice: Lattice = data.parse().unwrap();
        let lattice = lattice.drop(Axis::X);
        assert!(lattice.vertices.len() == 1);
    }

    #[test]
    fn drop_example_actually_dropping() {
        let data = r#"
            {
                "sites": [
                    {"kind": "Fe", "position": [0, 0, 0]}
                ],
                "vertices": [
                    {"source": 0, "target": 0, "delta": [0, 0, 1], "tags": ["core", "inner"]}
                ]
            }
        "#;
        let lattice: Lattice = data.parse().unwrap();
        let lattice = lattice.drop(Axis::Z);
        assert!(lattice.vertices.len() == 0);
    }

    #[test]
    fn single_lattice_expansion_1d() {
        let data = r#"
            {
                "sites": [
                    {"kind": "Fe", "position": [0, 0, 0]}
                ],
                "vertices": []
            }
        "#;
        let lattice: Lattice = data.parse().unwrap();
        let output = lattice.expand_along(Axis::X, 2);
        assert_eq!(output.sites.len(), 2);
        assert!((output.sites[1].position.0 - 1.0).abs() < 1e-10);
    }
}