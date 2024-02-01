//! Let's abstract an alloy

use rand::{distributions::WeightedIndex, prelude::Distribution, Rng};

/// An alloy is a collection of kinds of atoms and their ratios
#[derive(Debug)]
pub struct Alloy {
    kinds: Vec<String>,
    weights: WeightedIndex<u32>,
}

impl Alloy {
    pub fn new(kinds: Vec<&str>, ratios: Vec<u32>) -> Self {
        debug_assert!(kinds.len() == ratios.len());
        Self {
            kinds: kinds.into_iter().map(|s| s.to_owned()).collect(),
            weights: WeightedIndex::new(&ratios).unwrap(),
        }
    }

    pub fn pick<R: Rng + ?Sized>(&self, rng: &mut R) -> String {
        self.kinds[self.weights.sample(rng)].clone()
    }
}
