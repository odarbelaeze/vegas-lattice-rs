//! Let's abstract an alloy

use rand::{distributions::WeightedIndex, prelude::Distribution, Rng};

/// An alloy is a collection of kinds of atoms and their ratios
///
/// It is used to pick a kind of atom from the alloy.
///
/// # Example
///
/// ```rust
/// use vegas_lattice::Alloy;
///
/// let alloy = Alloy::new(vec!["Fe", "Ni"], vec![1, 2]);
/// let kind = alloy.pick(&mut rand::thread_rng());
///
/// assert!(kind == "Fe" || kind == "Ni");
/// ```
#[derive(Debug)]
pub struct Alloy {
    kinds: Vec<String>,
    weights: WeightedIndex<u32>,
}

impl Alloy {
    /// Create a new alloy with a given list of kinds and their ratios
    pub fn new(kinds: Vec<&str>, ratios: Vec<u32>) -> Self {
        debug_assert!(kinds.len() == ratios.len());
        Self {
            kinds: kinds.into_iter().map(|s| s.to_owned()).collect(),
            weights: WeightedIndex::new(&ratios).unwrap(),
        }
    }

    pub fn from_targets(targets: Vec<(&str, u32)>) -> Self {
        let (kinds, ratios): (Vec<_>, Vec<_>) = targets.into_iter().unzip();
        Self::new(kinds, ratios)
    }

    /// Picks a kind of atom from the alloy
    pub fn pick<R: Rng>(&self, rng: &mut R) -> String {
        self.kinds[self.weights.sample(rng)].clone()
    }
}
