//! Lattice data structure

use super::util::Axis;
use crate::{
    alloy::Alloy,
    edge::Edge,
    error::{Result, VegasLatticeError},
    mask::Mask,
    site::Site,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{iter::repeat_n, str::FromStr};

/// A lattice is a collection of sites and edges.
///
/// For now it only supports rectangular lattices. This is Orthorombic, Tetragonal and Cubic
/// Bravais lattices. We assume the lattice vectors are aligned with the cartesian axes. While you
/// can choose the lattice parameters _a_, _b_, and _c_ to be different.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Lattice {
    size: (f64, f64, f64),
    sites: Vec<Site>,
    edges: Vec<Edge>,
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
            edges: Vec::new(),
        })
    }

    /// Create a simple cubic lattice with the given size _a_
    pub fn sc(a: f64) -> Self {
        let sites = vec![Site::new("A")];
        let edges = vec![
            Edge::new(0, 0, (1, 0, 0)),
            Edge::new(0, 0, (0, 1, 0)),
            Edge::new(0, 0, (0, 0, 1)),
        ];
        Lattice {
            size: (a, a, a),
            sites,
            edges,
        }
    }

    /// Create a body centered cubic lattice with the given size _a_
    pub fn bcc(a: f64) -> Self {
        let sites = vec![
            Site::new("A"),
            Site::new("B").with_position((0.5 * a, 0.5 * a, 0.5 * a)),
        ];
        let edges = vec![
            Edge::new(0, 1, (0, 0, 0)),
            Edge::new(0, 1, (0, -1, 0)),
            Edge::new(0, 1, (-1, 0, 0)),
            Edge::new(0, 1, (-1, -1, 0)),
            Edge::new(0, 1, (0, 0, -1)),
            Edge::new(0, 1, (0, -1, -1)),
            Edge::new(0, 1, (-1, 0, -1)),
            Edge::new(0, 1, (-1, -1, -1)),
        ];
        Lattice {
            size: (a, a, a),
            sites,
            edges,
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
        let edges = vec![
            // xy plane
            Edge::new(0, 1, (0, 0, 0)),
            Edge::new(0, 1, (-1, 0, 0)),
            Edge::new(0, 1, (-1, -1, 0)),
            Edge::new(0, 1, (0, -1, 0)),
            // xz plane
            Edge::new(0, 2, (0, 0, 0)),
            Edge::new(0, 2, (-1, 0, 0)),
            Edge::new(0, 2, (-1, 0, -1)),
            Edge::new(0, 2, (0, 0, -1)),
            // yz plane
            Edge::new(0, 3, (0, 0, 0)),
            Edge::new(0, 3, (0, -1, 0)),
            Edge::new(0, 3, (0, -1, -1)),
            Edge::new(0, 3, (0, 0, -1)),
        ];
        Lattice {
            size: (a, a, a),
            sites,
            edges,
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

    /// Get the edges of the lattice
    pub fn edges(&self) -> &[Edge] {
        &self.edges
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

    /// Changes the edges of the lattice
    pub fn try_with_edges(mut self, edges: Vec<Edge>) -> Result<Self> {
        self.edges = edges;
        self.validate()
    }

    fn are_edges_consistent(&self) -> bool {
        self.edges
            .iter()
            .map(|edge| edge.source())
            .chain(self.edges.iter().map(|edge| edge.target()))
            .all(|id| id < self.sites.len())
    }

    /// Validates the lattice
    fn validate(self) -> Result<Self> {
        if !self.are_edges_consistent() {
            return Err(VegasLatticeError::InconsistentEdges);
        }
        if self.size.0 < 0.0 || self.size.1 < 0.0 || self.size.2 < 0.0 {
            return Err(VegasLatticeError::NegativeSize);
        }
        Ok(self)
    }

    /// Drops all the edges that are periodic along the given axis
    fn drop_along(mut self, axis: Axis) -> Self {
        self.edges.retain(|v| {
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
        let n_edges = self.edges.len();

        self.sites = (0..amount)
            .flat_map(|i| repeat_n(i, n_sites))
            .zip(self.sites().iter().cycle())
            .map(|(index, site)| match axis {
                Axis::X => site.clone().move_x((index as f64) * size),
                Axis::Y => site.clone().move_y((index as f64) * size),
                Axis::Z => site.clone().move_z((index as f64) * size),
            })
            .collect();

        self.edges = (0..amount)
            .flat_map(|i| repeat_n(i, n_edges))
            .zip(self.edges.iter().cycle())
            .map(|(index, edge)| match axis {
                Axis::X => edge.clone().move_x(index, n_sites, amount),
                Axis::Y => edge.clone().move_y(index, n_sites, amount),
                Axis::Z => edge.clone().move_z(index, n_sites, amount),
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

    /// Expand lattice by the same amount along all axes
    pub fn expand_all(self, amount: usize) -> Self {
        self.expand_x(amount).expand_y(amount).expand_z(amount)
    }

    /// Expand lattice by the given amount along all axes
    pub fn expand(self, x: usize, y: usize, z: usize) -> Self {
        self.expand_x(x).expand_y(y).expand_z(z)
    }

    /// Removes sites from the lattice according to the given mask and
    /// perpendicular to the given axis.
    fn apply_mask<R: Rng>(mut self, mask: Mask, axis: Axis, rng: &mut R) -> Self {
        let site_mask: Vec<_> = self
            .sites
            .iter()
            .map(|s| {
                let (x, y) = axis.project_in_plane(s.position());
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
        self.edges = self
            .edges
            .into_iter()
            .filter(|v| site_mask[v.source()] && site_mask[v.target()])
            .map(|v| v.reindex(&new_indices))
            .collect();
        self
    }

    // Apply a mask in the plan perpendicular to the x axis.
    pub fn apply_mask_x<R: Rng>(self, mask: Mask, rng: &mut R) -> Self {
        self.apply_mask(mask, Axis::X, rng)
    }

    // Apply a mask in the plan perpendicular to the y axis.
    pub fn apply_mask_y<R: Rng>(self, mask: Mask, rng: &mut R) -> Self {
        self.apply_mask(mask, Axis::Y, rng)
    }

    // Apply a mask in the plan perpendicular to the z axis.
    pub fn apply_mask_z<R: Rng>(self, mask: Mask, rng: &mut R) -> Self {
        self.apply_mask(mask, Axis::Z, rng)
    }

    /// Replaces the sites labeled as `source` with sites in the `target` alloy
    pub fn alloy_sites<R: Rng>(mut self, source: &str, target: Alloy, rng: &mut R) -> Self {
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
    use crate::{Edge, Lattice, Site};

    #[test]
    fn drop_example() {
        let lattice = Lattice::sc(1.0);
        let lattice = lattice.drop_x();
        assert!(lattice.edges().len() == 2);
    }

    #[test]
    fn drop_example_actually_dropping() {
        let lattice = Lattice::sc(1.0);
        let lattice = lattice.drop_all();
        assert!(lattice.edges().is_empty());
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
    fn single_lattice_expansion_1d_edges() {
        let lattice = Lattice::sc(1.0)
            .try_with_edges(vec![Edge::new(0, 0, (1, 0, 0))])
            .unwrap();
        let output = lattice.expand_x(2);
        assert_eq!(output.edges.len(), 2);
        assert_eq!(output.edges[0].source(), 0);
        assert_eq!(output.edges[0].target(), 1);
        assert_eq!(output.edges[0].delta().0, 0);
        assert_eq!(output.edges[1].source(), 1);
        assert_eq!(output.edges[1].target(), 0);
        assert_eq!(output.edges[1].delta().0, 1);
    }

    #[test]
    fn single_lattice_expansion_1d_negative_edges() {
        let lattice = Lattice::sc(1.0)
            .try_with_edges(vec![Edge::new(0, 0, (-1, 0, 0))])
            .unwrap();
        let output = lattice.expand_x(2);
        assert_eq!(output.edges.len(), 2);
        assert_eq!(output.edges[0].source(), 0);
        assert_eq!(output.edges[0].target(), 1);
        assert_eq!(output.edges[0].delta().0, -1);
        assert_eq!(output.edges[1].source(), 1);
        assert_eq!(output.edges[1].target(), 0);
        assert_eq!(output.edges[1].delta().0, 0);
    }

    #[test]
    fn test_sc_lattice() {
        let lattice = Lattice::sc(1.0);
        assert_eq!(lattice.size(), (1.0, 1.0, 1.0));
        assert_eq!(lattice.sites().len(), 1);
        assert_eq!(lattice.edges().len(), 3)
    }

    #[test]
    fn test_bcc_lattice() {
        let lattice = Lattice::bcc(1.0);
        assert_eq!(lattice.size(), (1.0, 1.0, 1.0));
        assert_eq!(lattice.sites().len(), 2);
        assert_eq!(lattice.edges().len(), 8)
    }

    #[test]
    fn test_fcc_lattice() {
        let lattice = Lattice::fcc(1.0);
        assert_eq!(lattice.size(), (1.0, 1.0, 1.0));
        assert_eq!(lattice.sites().len(), 4);
        assert_eq!(lattice.edges().len(), 12)
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
    fn test_with_edges() {
        let lattice = Lattice::sc(1.0)
            .try_with_edges(vec![Edge::new(0, 0, (1, 0, 0)), Edge::new(0, 0, (0, 1, 0))])
            .unwrap();
        assert_eq!(lattice.edges().len(), 2);
    }

    #[test]
    fn test_lattice_with_inconsistent_edges() {
        // The edge target is not in the list of sites
        let result = Lattice::sc(1.0).try_with_edges(vec![Edge::new(0, 1, (1, 0, 0))]);
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
            "edges": [
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
        assert_eq!(lattice.edges().len(), 1);
    }
}
