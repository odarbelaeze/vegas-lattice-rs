//! Error handling for the vegas lattice crate

use rand::distr::weighted::Error as WeightedError;
use serde_json::Error as SerdeError;
use std::io::Error as IoError;
use thiserror::Error;

/// Error type for the vegas lattice crate
#[derive(Error, Debug)]
pub enum VegasLatticeError {
    #[error("IO error: {0}")]
    IoError(#[from] IoError),
    #[error("serialization error: {0}")]
    SerializationError(#[from] SerdeError),
    #[error("Formatter error: {0}")]
    ImageReadError(#[from] image::ImageError),
    #[error("inconsistent vertices")]
    InconsistentVertices,
    #[error("negative size")]
    NegativeSize,
    #[error("inconsistent weights")]
    InconsistentWeights(#[from] WeightedError),
    #[error("invalid ratios")]
    InvalidRatios,
}

/// Result type for the vegas lattice crate
pub type Result<T> = std::result::Result<T, VegasLatticeError>;
