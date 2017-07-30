extern crate itertools;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod site;
mod util;
mod vertex;

use std::error::Error as StdError;
use std::fmt;

use itertools::Itertools;
use serde_json::Error as SerdeError;

use site::Site;
use vertex::Vertex;
pub use util::Axis;




#[derive(Debug, Serialize, Deserialize)]
pub struct Lattice {
    size: (f64, f64, f64),
    sites: Vec<Site>,
    vertices: Vec<Vertex>,
}


#[derive(Debug)]
pub enum LatticeError {
    JsonParseError(SerdeError),
    InconsistentVertices,
    NegativeSize,
}


impl fmt::Display for LatticeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LatticeError::JsonParseError(_) => f.write_str("JsonParseError"),
            LatticeError::InconsistentVertices => f.write_str("InconsistentVertices"),
            LatticeError::NegativeSize => f.write_str("NegativeSize"),
        }
    }
}


impl StdError for LatticeError {
    fn description(&self) -> &str {
        match *self {
            LatticeError::JsonParseError(_) => "Failed to parse JSON.",
            LatticeError::InconsistentVertices => "The vertices are inconsistent.",
            LatticeError::NegativeSize => "A negative size value is hard to grasp.",
        }
    }
}


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
        if self.size.0 < 0.0 || self.size.1 < 0.0 || self.size.2 < 0.0 {
            return Err(LatticeError::NegativeSize);
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

    pub fn size_along(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => self.size.0,
            Axis::Y => self.size.1,
            Axis::Z => self.size.2,
        }
    }

    pub fn expand_along(mut self, axis: Axis, amount: usize) -> Self {
        let size = self.size_along(axis);
        let n_sites = self.sites.len();

        self.sites = (0..amount)
            .cartesian_product(self.sites)
            .map(|(index, site)| site.move_along(axis, (index as f64) * size))
            .collect();

        self.vertices = (0..amount)
            .cartesian_product(self.vertices)
            .map(|(index, vertex)| vertex.move_along(axis, index, n_sites, amount))
            .collect();

        match axis {
            Axis::X => self.size.0 *= amount as f64,
            Axis::Y => self.size.1 *= amount as f64,
            Axis::Z => self.size.2 *= amount as f64,
        }

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
    use super::{Vertex, Lattice, Axis};

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
                "size": [1, 1, 1],
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
                "size": [1, 1, 1],
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
                "size": [1, 1, 1],
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

    #[test]
    fn double_lattice_expansion_1d() {
        let data = r#"
            {
                "size": [1, 1, 1],
                "sites": [
                    {"kind": "Fe", "position": [0, 0, 0]}
                ],
                "vertices": []
            }
        "#;
        let lattice: Lattice = data.parse().unwrap();
        let lattice = lattice.expand_along(Axis::X, 2);
        let output = lattice.expand_along(Axis::X, 2);
        assert_eq!(output.sites.len(), 4);
        assert!((output.sites[1].position.0 - 1.0).abs() < 1e-10);
        assert!((output.sites[2].position.0 - 2.0).abs() < 1e-10);
        assert!((output.sites[3].position.0 - 3.0).abs() < 1e-10);
    }

    #[test]
    fn single_lattice_expansion_1d_vertices() {
        let data = r#"
            {
                "size": [1, 1, 1],
                "sites": [
                    {"kind": "Fe", "position": [0, 0, 0]}
                ],
                "vertices": [
                    {"source": 0, "target": 0, "delta": [1, 0, 0]}
                ]
            }
        "#;
        let lattice: Lattice = data.parse().unwrap();
        let output = lattice.expand_along(Axis::X, 2);
        assert_eq!(output.vertices.len(), 2);
        assert_eq!(output.vertices[0].source, 0);
        assert_eq!(output.vertices[0].target, 1);
        assert_eq!(output.vertices[0].delta.0, 0);
        assert_eq!(output.vertices[1].source, 1);
        assert_eq!(output.vertices[1].target, 0);
        assert_eq!(output.vertices[1].delta.0, 1);
    }

    #[test]
    fn single_lattice_expansion_1d_negative_vertices() {
        let data = r#"
            {
                "size": [1, 1, 1],
                "sites": [
                    {"kind": "Fe", "position": [0, 0, 0]}
                ],
                "vertices": [
                    {"source": 0, "target": 0, "delta": [-1, 0, 0]}
                ]
            }
        "#;
        let lattice: Lattice = data.parse().unwrap();
        let output = lattice.expand_along(Axis::X, 2);
        assert_eq!(output.vertices.len(), 2);
        assert_eq!(output.vertices[0].source, 0);
        assert_eq!(output.vertices[0].target, 1);
        assert_eq!(output.vertices[0].delta.0, -1);
        assert_eq!(output.vertices[1].source, 1);
        assert_eq!(output.vertices[1].target, 0);
        assert_eq!(output.vertices[1].delta.0, 0);
    }
}