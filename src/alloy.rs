//! Let's abstract an alloy

use crate::error::{Result, VegasLatticeError};
use rand::{distr::weighted::WeightedIndex, distr::Distribution, Rng};

/// An alloy is a collection of kinds of atoms and their ratios
///
/// It is used to pick a kind of atom from the alloy.
///
/// # Example
///
/// ```rust
/// use vegas_lattice::Alloy;
///
/// let alloy = Alloy::try_new(vec!["Fe", "Ni"], vec![1, 2]).unwrap();
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
    pub fn try_new(kinds: Vec<&str>, ratios: Vec<u32>) -> Result<Self> {
        if kinds.len() != ratios.len() {
            return Err(VegasLatticeError::InvalidRatios);
        }
        let kinds = kinds.into_iter().map(|s| s.to_owned()).collect();
        let weights = WeightedIndex::new(&ratios)?;
        Ok(Self { kinds, weights })
    }

    pub fn try_from_targets(targets: Vec<(&str, u32)>) -> Result<Self> {
        let (kinds, ratios): (Vec<_>, Vec<_>) = targets.into_iter().unzip();
        Self::try_new(kinds, ratios)
    }

    /// Picks a kind of atom from the alloy
    pub fn pick<R: Rng + ?Sized>(&self, rng: &mut R) -> &str {
        &self.kinds[self.weights.sample(rng)]
    }
}
