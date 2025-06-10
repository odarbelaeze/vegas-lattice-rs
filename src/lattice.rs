//! Lattice data structure

use super::util::Axis;
use crate::alloy::Alloy;
use crate::error::{Result, VegasLatticeError};
use crate::mask::Mask;
use crate::site::Site;
use crate::vertex::Vertex;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::iter::repeat;
use std::str::FromStr;

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
    pub fn try_new(size: (f64, f64, f64)) -> Result<Self> {
        if size.0 < 0.0 || size.1 < 0.0 || size.2 < 0.0 {
            return Err(VegasLatticeError::NegativeSize);
        }
        Ok(Lattice {
            size,
            sites: Vec::new(),
            vertices: Vec::new(),
        })
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
        let sites = vec![
            Site::new("A"),
            Site::new("B").with_position((0.5 * a, 0.5 * a, 0.5 * a)),
        ];
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

    /// Create a face centered cubic lattice with lattice parameter _a_
    pub fn fcc(a: f64) -> Self {
        let sites = vec![
            Site::new("A"),
            Site::new("B").with_position((0.5 * a, 0.5 * a, 0.0)),
            Site::new("C").with_position((0.5 * a, 0.0, 0.5 * a)),
            Site::new("D").with_position((0.0, 0.5 * a, 0.5 * a)),
        ];
        let vertices = vec![
            // xy plane
            Vertex::new(0, 1, (0, 0, 0)),
            Vertex::new(0, 1, (-1, 0, 0)),
            Vertex::new(0, 1, (-1, -1, 0)),
            Vertex::new(0, 1, (0, -1, 0)),
            // xz plane
            Vertex::new(0, 2, (0, 0, 0)),
            Vertex::new(0, 2, (-1, 0, 0)),
            Vertex::new(0, 2, (-1, 0, -1)),
            Vertex::new(0, 2, (0, 0, -1)),
            // yz plane
            Vertex::new(0, 3, (0, 0, 0)),
            Vertex::new(0, 3, (0, -1, 0)),
            Vertex::new(0, 3, (0, -1, -1)),
            Vertex::new(0, 3, (0, 0, -1)),
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

    /// Changes the size of the lattice
    pub fn try_with_size(mut self, size: (f64, f64, f64)) -> Result<Self> {
        self.size = size;
        self.validate()
    }

    /// Changes the sites of the lattice
    pub fn try_with_sites(mut self, sites: Vec<Site>) -> Result<Self> {
        self.sites = sites;
        self.validate()
    }

    /// Changes the vertices of the lattice
    pub fn try_with_vertices(mut self, vertices: Vec<Vertex>) -> Result<Self> {
        self.vertices = vertices;
        self.validate()
    }

    fn are_vertices_consistent(&self) -> bool {
        self.vertices
            .iter()
            .map(|vertex| vertex.source())
            .chain(self.vertices.iter().map(|vertex| vertex.target()))
            .all(|id| id < self.sites.len())
    }

    /// Validates the lattice
    fn validate(self) -> Result<Self> {
        if !self.are_vertices_consistent() {
            return Err(VegasLatticeError::InconsistentVertices);
        }
        if self.size.0 < 0.0 || self.size.1 < 0.0 || self.size.2 < 0.0 {
            return Err(VegasLatticeError::NegativeSize);
        }
        Ok(self)
    }

    /// Drops all the vertices that are periodic along the given axis
    fn drop_along(mut self, axis: Axis) -> Self {
        self.vertices.retain(|v| {
            let delta = v.delta();
            match axis {
                Axis::X => delta.0 == 0,
                Axis::Y => delta.1 == 0,
                Axis::Z => delta.2 == 0,
            }
        });
        self
    }

    /// Drop periodic boundary conditions along the x axis
    pub fn drop_x(self) -> Self {
        self.drop_along(Axis::X)
    }

    /// Drop periodic boundary conditions along the y axis
    pub fn drop_y(self) -> Self {
        self.drop_along(Axis::Y)
    }

    /// Drop periodic boundary conditions along the z axis
    pub fn drop_z(self) -> Self {
        self.drop_along(Axis::Z)
    }

    /// Drop periodic boundary conditions along all axes
    pub fn drop_all(self) -> Self {
        self.drop_x().drop_y().drop_z()
    }

    #[inline]
    fn size_along(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => self.size.0,
            Axis::Y => self.size.1,
            Axis::Z => self.size.2,
        }
    }

    /// Expands the lattice along the given axis
    fn expand_along(mut self, axis: Axis, amount: usize) -> Self {
        let size = self.size_along(axis);
        let n_sites = self.sites.len();
        let n_vertices = self.vertices.len();

        self.sites = (0..amount)
            .flat_map(|i| repeat(i).take(n_sites))
            .zip(self.sites().iter().cycle())
            .map(|(index, site)| match axis {
                Axis::X => site.clone().move_x((index as f64) * size),
                Axis::Y => site.clone().move_y((index as f64) * size),
                Axis::Z => site.clone().move_z((index as f64) * size),
            })
            .collect();

        self.vertices = (0..amount)
            .flat_map(|i| repeat(i).take(n_vertices))
            .zip(self.vertices.iter().cycle())
            .map(|(index, vertex)| match axis {
                Axis::X => vertex.clone().move_x(index, n_sites, amount),
                Axis::Y => vertex.clone().move_y(index, n_sites, amount),
                Axis::Z => vertex.clone().move_z(index, n_sites, amount),
            })
            .collect();

        match axis {
            Axis::X => self.size.0 *= amount as f64,
            Axis::Y => self.size.1 *= amount as f64,
            Axis::Z => self.size.2 *= amount as f64,
        }

        self
    }

    /// Expand lattice along the x axis
    pub fn expand_x(self, amount: usize) -> Self {
        self.expand_along(Axis::X, amount)
    }

    /// Expand lattice along the y axis
    pub fn expand_y(self, amount: usize) -> Self {
        self.expand_along(Axis::Y, amount)
    }

    /// Expand lattice along the z axis
    pub fn expand_z(self, amount: usize) -> Self {
        self.expand_along(Axis::Z, amount)
    }

    /// Expand lattice by the same ammount along all axes
    pub fn expand_all(self, amount: usize) -> Self {
        self.expand_x(amount).expand_y(amount).expand_z(amount)
    }

    /// Expand lattice by the given ammount along all axes
    pub fn expand(self, x: usize, y: usize, z: usize) -> Self {
        self.expand_x(x).expand_y(y).expand_z(z)
    }

    /// Removes sites from the lattice according to the given mask
    ///
    /// TODO: This only removes points in the xy plane, and it should be generalized
    pub fn apply_mask<R: Rng + ?Sized>(mut self, mask: Mask, rng: &mut R) -> Self {
        let site_mask: Vec<_> = self
            .sites
            .iter()
            .map(|s| {
                let (x, y, _) = s.position();
                mask.keep(x, y, rng)
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
    pub fn alloy_sites<R: Rng + ?Sized>(
        mut self,
        source: &str,
        target: Alloy,
        rng: &mut R,
    ) -> Self {
        self.sites = self
            .sites
            .into_iter()
            .map(|site| {
                if site.kind() != source {
                    site
                } else {
                    site.with_kind(target.pick(rng))
                }
            })
            .collect();
        self
    }
}

impl FromStr for Lattice {
    type Err = VegasLatticeError;
    fn from_str(source: &str) -> Result<Lattice> {
        let lattice: Lattice = serde_json::from_str(source)?;
        lattice.validate()
    }
}

#[cfg(test)]
mod test {
    use crate::{Lattice, Site, Vertex};

    #[test]
    fn drop_example() {
        let lattice = Lattice::sc(1.0);
        let lattice = lattice.drop_x();
        assert!(lattice.vertices().len() == 2);
    }

    #[test]
    fn drop_example_actually_dropping() {
        let lattice = Lattice::sc(1.0);
        let lattice = lattice.drop_all();
        assert!(lattice.vertices().is_empty());
    }

    #[test]
    fn single_lattice_expansion_1d() {
        let lattice = Lattice::sc(1.0);
        let output = lattice.expand_x(2);
        assert_eq!(output.sites.len(), 2);
        assert!((output.sites[1].position().0 - 1.0).abs() < 1e-10);
    }

    #[test]
    fn double_lattice_expansion_1d() {
        let lattice = Lattice::sc(1.0);
        let lattice = lattice.expand_x(2);
        let output = lattice.expand_x(2);
        assert_eq!(output.sites.len(), 4);
        assert!((output.sites[1].position().0 - 1.0).abs() < 1e-10);
        assert!((output.sites[2].position().0 - 2.0).abs() < 1e-10);
        assert!((output.sites[3].position().0 - 3.0).abs() < 1e-10);
    }

    #[test]
    fn single_lattice_expansion_1d_vertices() {
        let lattice = Lattice::sc(1.0)
            .try_with_vertices(vec![Vertex::new(0, 0, (1, 0, 0))])
            .unwrap();
        let output = lattice.expand_x(2);
        assert_eq!(output.vertices.len(), 2);
        assert_eq!(output.vertices[0].source(), 0);
        assert_eq!(output.vertices[0].target(), 1);
        assert_eq!(output.vertices[0].delta().0, 0);
        assert_eq!(output.vertices[1].source(), 1);
        assert_eq!(output.vertices[1].target(), 0);
        assert_eq!(output.vertices[1].delta().0, 1);
    }

    #[test]
    fn single_lattice_expansion_1d_negative_vertices() {
        let lattice = Lattice::sc(1.0)
            .try_with_vertices(vec![Vertex::new(0, 0, (-1, 0, 0))])
            .unwrap();
        let output = lattice.expand_x(2);
        assert_eq!(output.vertices.len(), 2);
        assert_eq!(output.vertices[0].source(), 0);
        assert_eq!(output.vertices[0].target(), 1);
        assert_eq!(output.vertices[0].delta().0, -1);
        assert_eq!(output.vertices[1].source(), 1);
        assert_eq!(output.vertices[1].target(), 0);
        assert_eq!(output.vertices[1].delta().0, 0);
    }

    #[test]
    fn test_sc_lattice() {
        let lattice = Lattice::sc(1.0);
        assert_eq!(lattice.size(), (1.0, 1.0, 1.0));
        assert_eq!(lattice.sites().len(), 1);
        assert_eq!(lattice.vertices().len(), 3)
    }

    #[test]
    fn test_bcc_lattice() {
        let lattice = Lattice::bcc(1.0);
        assert_eq!(lattice.size(), (1.0, 1.0, 1.0));
        assert_eq!(lattice.sites().len(), 2);
        assert_eq!(lattice.vertices().len(), 8)
    }

    #[test]
    fn test_fcc_lattice() {
        let lattice = Lattice::fcc(1.0);
        assert_eq!(lattice.size(), (1.0, 1.0, 1.0));
        assert_eq!(lattice.sites().len(), 4);
        assert_eq!(lattice.vertices().len(), 12)
    }

    #[test]
    fn test_wit_size() {
        let lattice = Lattice::sc(1.0).try_with_size((2.0, 2.0, 2.0)).unwrap();
        assert_eq!(lattice.size(), (2.0, 2.0, 2.0));
    }

    #[test]
    fn test_with_sites() {
        let lattice = Lattice::sc(1.0)
            .try_with_sites(vec![Site::new("Fe"), Site::new("Ni")])
            .unwrap();
        assert_eq!(lattice.sites().len(), 2);
    }

    #[test]
    fn test_with_vertices() {
        let lattice = Lattice::sc(1.0)
            .try_with_vertices(vec![
                Vertex::new(0, 0, (1, 0, 0)),
                Vertex::new(0, 0, (0, 1, 0)),
            ])
            .unwrap();
        assert_eq!(lattice.vertices().len(), 2);
    }

    #[test]
    fn test_lattice_with_inconsistent_vertices() {
        // The vertex target is not in the list of sites
        let result = Lattice::sc(1.0).try_with_vertices(vec![Vertex::new(0, 1, (1, 0, 0))]);
        assert!(result.is_err());
    }

    #[test]
    fn test_lattice_with_negative_size() {
        let result = Lattice::try_new((-1.0, 1.0, 1.0));
        assert!(result.is_err());
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
