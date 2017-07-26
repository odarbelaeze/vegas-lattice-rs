extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use serde_json::Error;
use std::collections::HashSet;
use std::error::Error as StdError;
use std::fmt;


#[derive(Debug, Serialize, Deserialize)]
pub struct Site {
    id: u64,
    kind: String,
    position: (f64, f64, f64),
    tags: Option<Vec<String>>,
}


impl std::str::FromStr for Site {
    type Err = Error;
    fn from_str(source: &str) -> Result<Site, Self::Err> {
        serde_json::from_str(source)
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Vertex {
    source: u64,
    target: u64,
    delta: (i32, i32, i32),
    tags: Option<Vec<String>>,
}


impl std::str::FromStr for Vertex {
    type Err = Error;
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
    JsonParseError(Error),
    InconsistentVertices,
    NonUniqueSiteIds,
}


impl fmt::Display for LatticeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LatticeError::JsonParseError(_) => f.write_str("JsonParseError"),
            LatticeError::InconsistentVertices => f.write_str("InconsistentVertices"),
            LatticeError::NonUniqueSiteIds => f.write_str("NonUniqueIds"),
        }
    }
}


impl StdError for LatticeError {
    fn description(&self) -> &str {
        match *self {
            LatticeError::JsonParseError(_) => "Failed to parse JSON.",
            LatticeError::InconsistentVertices => "The vertices are inconsistent.",
            LatticeError::NonUniqueSiteIds => "The ids of the sites are not unique.",
        }
    }
}


pub enum Axis { X, Y, Z }


impl Lattice {
    fn are_vertices_consistent(&self) -> bool {
        let site_ids: HashSet<_> = self.sites
            .iter()
            .map(|site| site.id)
            .collect();
        let vertex_ids: HashSet<_> = self.vertices
            .iter()
            .map(|vertex| vertex.source)
            .chain(self.vertices.iter().map(|vertex| vertex.target))
            .collect();
        vertex_ids.is_subset(&site_ids)
    }

    fn are_site_ids_unique(&self) -> bool {
        let site_ids: Vec<_> = self.sites
            .iter()
            .map(|site| site.id)
            .collect();
        let unique_site_ids: HashSet<_> = site_ids.iter().collect();
        site_ids.len() == unique_site_ids.len()
    }

    pub fn validate(self) -> Result<Self, LatticeError> {
        if !self.are_site_ids_unique() {
            return Err(LatticeError::NonUniqueSiteIds);
        }
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
            {"id": 0, "kind": "Fe", "position": [0, 0, 0], "tags": ["core", "inner"]}
        "#;
        let site_result: Result<Site, _> = data.parse();
        assert!(site_result.is_ok());
        assert_eq!(site_result.unwrap().tags,
                   Some(vec!["core".to_string(), "inner".to_string()]));
    }

    #[test]
    fn site_will_parse_id() {
        let data = r#"
            {"id": 0, "kind": "Fe", "position": [0, 0, 0]}
        "#;
        let site_result: Result<Site, _> = data.parse();
        assert!(site_result.is_ok());
        assert_eq!(site_result.unwrap().id, 0);
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
                    {"id": 0, "kind": "Fe", "position": [0, 0, 0]}
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
                    {"id": 0, "kind": "Fe", "position": [0, 0, 0]}
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
}