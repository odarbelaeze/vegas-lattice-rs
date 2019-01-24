extern crate serde_json;

use std::str::FromStr;

use itertools::Itertools;
use rand::distributions::{WeightedChoice, IndependentSample};
use rand::thread_rng;

use super::error::LatticeError;
use super::mask::Mask;
use super::site::Site;
use super::util::Axis;
use super::vertex::Vertex;
use super::alloy::Alloy;


#[derive(Debug, Serialize, Deserialize)]
pub struct Lattice {
    size: (f64, f64, f64),
    sites: Vec<Site>,
    vertices: Vec<Vertex>,
}


impl Lattice {

    pub fn size(&self) -> (f64, f64, f64) {
        self.size
    }

    pub fn sites(&self) -> &[Site] {
        &self.sites
    }

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
        self.vertices = self.vertices
            .into_iter()
            .filter(|v| v.delta_along(axis) == 0)
            .collect();
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

    pub fn apply_mask(mut self, mut mask: Mask) -> Self {
        let site_mask: Vec<_> = self.sites
            .iter()
            .map(|s| {
                let (x, y, _) = s.position();
                mask.keep(x, y)
            })
            .collect();
        let mut counter = 0;
        let new_indices: Vec<_> = (0..self.sites.len())
            .map(|i| if site_mask[i] { counter += 1; counter - 1 } else { i })
            .collect();
        self.sites = self.sites
            .into_iter()
            .enumerate()
            .filter(|&(i, ref _s)| site_mask[i])
            .map(|(_i, s)| s)
            .collect();
        self.vertices = self.vertices
            .into_iter()
            .filter(|v| site_mask[v.source()] && site_mask[v.target()])
            .map(|v| v.reindex(&new_indices))
            .collect();
        self
    }

    pub fn alloy_sites(mut self, source: &str, target: Alloy) -> Self {
        let mut items = target.choices();
        let mut rng = thread_rng();
        let weigthed_choice = WeightedChoice::new(&mut items);
        self.sites = self.sites
            .into_iter()
            .map(|site| if site.kind() != source { site } else {
                site.with_kind(weigthed_choice.ind_sample(&mut rng))
            })
            .collect();
        self
    }
}


impl FromStr for Lattice {
    type Err = LatticeError;
    fn from_str(source: &str) -> Result<Lattice, Self::Err> {
        serde_json::from_str(source)
            .map_err(LatticeError::JsonParseError)
            .and_then(|lattice: Lattice| lattice.validate())
    }
}



#[cfg(test)]
mod test {
    use super::{Vertex, Lattice, Axis};
    use util::Tagged;

    #[test]
    fn vertex_will_take_optional_tags() {
        let data = r#"
            {"source": 0, "target": 0, "delta": [0, 0, 1], "tags": ["core", "inner"]}
        "#;
        let site_result: Result<Vertex, _> = data.parse();
        assert!(site_result.is_ok());
        assert_eq!(site_result.unwrap().tags(),
                   Some(&vec!["core".to_string(), "inner".to_string()]));
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
