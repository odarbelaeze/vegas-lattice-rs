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

/// A lattice is a collection of sites and vertices.
///
/// For now it only supports rectangular lattices. This is Orthorombic, Tetragonal and Cubic
/// Bravais lattices. We assume the lattice vectors are aligned with the cartesian axes. While you
/// can choose the lattice parameters _a_, _b_, and _c_ to be different.
#[derive(Debug, Serialize, Deserialize)]
pub struct Lattice {
    size: (f64, f64, f64),
    sites: Vec<Site>,
    vertices: Vec<Vertex>,
}

impl Lattice {
    /// Create a new lattice with the given size
    pub fn new(size: (f64, f64, f64)) -> Self {
        Lattice {
            size,
            sites: Vec::new(),
            vertices: Vec::new(),
        }
    }

    /// Create a simple cubic lattice with the given size _a_
    pub fn sc(a: f64) -> Self {
        let sites = vec![Site::new("A")];
        let vertices = vec![
            Vertex::new(0, 0, (1, 0, 0)),
            Vertex::new(0, 0, (0, 1, 0)),
            Vertex::new(0, 0, (0, 0, 1)),
        ];
        Lattice {
            size: (a, a, a),
            sites,
            vertices,
        }
    }

    /// Create a body centered cubic lattice with the given size _a_
    pub fn bcc(a: f64) -> Self {
        let sites = vec![Site::new("A"), Site::new("B")];
        let vertices = vec![
            Vertex::new(0, 1, (0, 0, 0)),
            Vertex::new(0, 1, (0, -1, 0)),
            Vertex::new(0, 1, (-1, 0, 0)),
            Vertex::new(0, 1, (-1, -1, 0)),
            Vertex::new(0, 1, (0, 0, -1)),
            Vertex::new(0, 1, (0, -1, -1)),
            Vertex::new(0, 1, (-1, 0, -1)),
            Vertex::new(0, 1, (-1, -1, -1)),
        ];
        Lattice {
            size: (a, a, a),
            sites,
            vertices,
        }
    }

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

    /// Returns the size of the lattice along the given axis
    pub fn size_along(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => self.size.0,
            Axis::Y => self.size.1,
            Axis::Z => self.size.2,
        }
    }

    /// Changes the size of the lattice
    pub fn with_size(mut self, size: (f64, f64, f64)) -> Self {
        self.size = size;
        self
    }

    /// Changes the sites of the lattice
    pub fn with_sites(mut self, sites: Vec<Site>) -> Self {
        self.sites = sites;
        self
    }

    /// Changes the vertices of the lattice
    pub fn with_vertices(mut self, vertices: Vec<Vertex>) -> Self {
        self.vertices = vertices;
        self
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

    /// Expands the lattice along the given axis
    pub fn expand_along(mut self, axis: Axis, amount: usize) -> Self {
        let size = self.size_along(axis);
        let n_sites = self.sites.len();
        let n_vertices = self.vertices.len();

        self.sites = (0..amount)
            .flat_map(|i| repeat(i).take(n_sites))
            .zip(self.sites().iter().cycle())
            .map(|(index, site)| site.clone().move_along(axis, (index as f64) * size))
            .collect();

        self.vertices = (0..amount)
            .flat_map(|i| repeat(i).take(n_vertices))
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
    pub fn apply_mask(mut self, mask: Mask) -> Self {
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
    use crate::{Axis, Lattice, Site, Vertex};

    #[test]
    fn drop_example() {
        let lattice = Lattice::new((1.0, 1.0, 1.0))
            .with_sites(vec![Site::new("Fe")])
            .with_vertices(vec![Vertex::new(0, 0, (0, 0, 1))]);
        let lattice = lattice.drop(Axis::X);
        assert!(lattice.vertices.len() == 1);
    }

    #[test]
    fn drop_example_actually_dropping() {
        let lattice = Lattice::new((1.0, 1.0, 1.0))
            .with_sites(vec![Site::new("Fe")])
            .with_vertices(vec![Vertex::new(0, 0, (0, 0, 1))]);
        let lattice = lattice.drop(Axis::Z);
        assert!(lattice.vertices.is_empty());
    }

    #[test]
    fn single_lattice_expansion_1d() {
        let lattice = Lattice::new((1.0, 1.0, 1.0)).with_sites(vec![Site::new("Fe")]);
        let output = lattice.expand_along(Axis::X, 2);
        assert_eq!(output.sites.len(), 2);
        assert!((output.sites[1].position().0 - 1.0).abs() < 1e-10);
    }

    #[test]
    fn double_lattice_expansion_1d() {
        let lattice = Lattice::new((1.0, 1.0, 1.0)).with_sites(vec![Site::new("Fe")]);
        let lattice = lattice.expand_along(Axis::X, 2);
        let output = lattice.expand_along(Axis::X, 2);
        assert_eq!(output.sites.len(), 4);
        assert!((output.sites[1].position().0 - 1.0).abs() < 1e-10);
        assert!((output.sites[2].position().0 - 2.0).abs() < 1e-10);
        assert!((output.sites[3].position().0 - 3.0).abs() < 1e-10);
    }

    #[test]
    fn single_lattice_expansion_1d_vertices() {
        let lattice = Lattice::new((1.0, 1.0, 1.0))
            .with_sites(vec![Site::new("Fe")])
            .with_vertices(vec![Vertex::new(0, 0, (1, 0, 0))]);
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
        let lattice = Lattice::new((1.0, 1.0, 1.0))
            .with_sites(vec![Site::new("Fe")])
            .with_vertices(vec![Vertex::new(0, 0, (-1, 0, 0))]);
        let output = lattice.expand_along(Axis::X, 2);
        assert_eq!(output.vertices.len(), 2);
        assert_eq!(output.vertices[0].source(), 0);
        assert_eq!(output.vertices[0].target(), 1);
        assert_eq!(output.vertices[0].delta_along(Axis::X), -1);
        assert_eq!(output.vertices[1].source(), 1);
        assert_eq!(output.vertices[1].target(), 0);
        assert_eq!(output.vertices[1].delta_along(Axis::X), 0);
    }

    #[test]
    fn test_lattice_can_be_read_from_string() {
        let lattice = r#"{
            "size": [1.0, 1.0, 1.0],
            "sites": [
                {
                    "kind": "Fe",
                    "position": [0.0, 0.0, 0.0]
                }
            ],
            "vertices": [
                {
                    "source": 0,
                    "target": 0,
                    "delta": [0, 0, 1]
                }
            ]
        }"#;
        let lattice: Lattice = lattice.parse().unwrap();
        assert_eq!(lattice.size(), (1.0, 1.0, 1.0));
        assert_eq!(lattice.sites().len(), 1);
        assert_eq!(lattice.vertices().len(), 1);
    }
}
