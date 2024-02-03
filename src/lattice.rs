//! Lattice data structure
use std::iter::repeat;
use std::str::FromStr;

use rand::thread_rng;

use super::alloy::Alloy;
use super::error::LatticeError;
use super::mask::Mask;
use super::site::Site;
use super::util::Axis;
use super::vertex::Vertex;
use serde::{Deserialize, Serialize};

/// A lattice is a collection of sites and vertices
#[derive(Debug, Serialize, Deserialize)]
pub struct Lattice {
    size: (f64, f64, f64),
    sites: Vec<Site>,
    vertices: Vec<Vertex>,
}

impl Lattice {
    /// Get the size of the lattice
    pub fn size(&self) -> (f64, f64, f64) {
        self.size
    }

    /// Get the sites of the lattice
    pub fn sites(&self) -> &[Site] {
        &self.sites
    }

    /// Get the vertices of the lattice
    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    fn are_vertices_consistent(&self) -> bool {
        self.vertices
            .iter()
            .map(|vertex| vertex.source())
            .chain(self.vertices.iter().map(|vertex| vertex.target()))
            .all(|id| id < self.sites.len())
    }

    /// Validates the lattice
    pub fn validate(self) -> Result<Self, LatticeError> {
        if !self.are_vertices_consistent() {
            return Err(LatticeError::InconsistentVertices);
        }
        if self.size.0 < 0.0 || self.size.1 < 0.0 || self.size.2 < 0.0 {
            return Err(LatticeError::NegativeSize);
        }
        Ok(self)
    }

    /// Drops all the vertices that are periodic along the given axis
    pub fn drop(mut self, axis: Axis) -> Self {
        self.vertices.retain(|v| v.delta_along(axis) == 0);
        self
    }

    /// Returns the size of the lattice along the given axis
    pub fn size_along(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => self.size.0,
            Axis::Y => self.size.1,
            Axis::Z => self.size.2,
        }
    }

    /// Expands the lattice along the given axis
    pub fn expand_along(mut self, axis: Axis, amount: usize) -> Self {
        let size = self.size_along(axis);
        let n_sites = self.sites.len();

        self.sites = (0..amount)
            .flat_map(|i| repeat(i).take(n_sites))
            .zip(self.sites().iter().cycle())
            .map(|(index, site)| site.clone().move_along(axis, (index as f64) * size))
            .collect();

        self.vertices = (0..amount)
            .flat_map(|i| repeat(i).take(n_sites))
            .zip(self.vertices.iter().cycle())
            .map(|(index, vertex)| vertex.clone().move_along(axis, index, n_sites, amount))
            .collect();

        match axis {
            Axis::X => self.size.0 *= amount as f64,
            Axis::Y => self.size.1 *= amount as f64,
            Axis::Z => self.size.2 *= amount as f64,
        }

        self
    }

    /// Removes sites from the lattice according to the given mask
    ///
    /// TODO: This only removes points in the xy plane, and it should be generalized
    pub fn apply_mask(mut self, mut mask: Mask) -> Self {
        let mut rng = thread_rng();
        let site_mask: Vec<_> = self
            .sites
            .iter()
            .map(|s| {
                let (x, y, _) = s.position();
                mask.keep(x, y, &mut rng)
            })
            .collect();
        let mut counter = 0;
        let new_indices: Vec<_> = (0..self.sites.len())
            .map(|i| {
                if site_mask[i] {
                    counter += 1;
                    counter - 1
                } else {
                    i
                }
            })
            .collect();
        self.sites = self
            .sites
            .into_iter()
            .enumerate()
            .filter(|&(i, ref _s)| site_mask[i])
            .map(|(_i, s)| s)
            .collect();
        self.vertices = self
            .vertices
            .into_iter()
            .filter(|v| site_mask[v.source()] && site_mask[v.target()])
            .map(|v| v.reindex(&new_indices))
            .collect();
        self
    }

    /// Replaces the sites labeled as `source` with sites in the `target` alloy
    pub fn alloy_sites(mut self, source: &str, target: Alloy) -> Self {
        let mut rng = thread_rng();
        self.sites = self
            .sites
            .into_iter()
            .map(|site| {
                if site.kind() != source {
                    site
                } else {
                    site.with_kind(target.pick(&mut rng))
                }
            })
            .collect();
        self
    }
}

impl FromStr for Lattice {
    type Err = LatticeError;
    fn from_str(source: &str) -> Result<Lattice, Self::Err> {
        let lattice: Lattice = serde_json::from_str(source)?;
        lattice.validate()
    }
}

#[cfg(test)]
mod test {
    use super::{Axis, Lattice, Vertex};
    use util::Tagged;

    #[test]
    fn vertex_will_take_optional_tags() {
        let data = r#"
            {"source": 0, "target": 0, "delta": [0, 0, 1], "tags": ["core", "inner"]}
        "#;
        let site_result: Result<Vertex, _> = data.parse();
        assert!(site_result.is_ok());
        assert_eq!(
            site_result.unwrap().tags(),
            Some(&vec!["core".to_string(), "inner".to_string()])
        );
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
        assert!(lattice.vertices.is_empty());
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
        assert!((output.sites[1].position().0 - 1.0).abs() < 1e-10);
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
        assert!((output.sites[1].position().0 - 1.0).abs() < 1e-10);
        assert!((output.sites[2].position().0 - 2.0).abs() < 1e-10);
        assert!((output.sites[3].position().0 - 3.0).abs() < 1e-10);
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
        assert_eq!(output.vertices[0].source(), 0);
        assert_eq!(output.vertices[0].target(), 1);
        assert_eq!(output.vertices[0].delta_along(Axis::X), 0);
        assert_eq!(output.vertices[1].source(), 1);
        assert_eq!(output.vertices[1].target(), 0);
        assert_eq!(output.vertices[1].delta_along(Axis::X), 1);
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
        assert_eq!(output.vertices[0].source(), 0);
        assert_eq!(output.vertices[0].target(), 1);
        assert_eq!(output.vertices[0].delta_along(Axis::X), -1);
        assert_eq!(output.vertices[1].source(), 1);
        assert_eq!(output.vertices[1].target(), 0);
        assert_eq!(output.vertices[1].delta_along(Axis::X), 0);
    }
}
