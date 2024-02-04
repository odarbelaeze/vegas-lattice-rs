//! Error handling for the lattice crate

use serde_json::Error as SerdeError;
use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IoError;

/// Error type for the lattice crate
#[derive(Debug)]
pub enum LatticeError {
    IoError(IoError),
    JsonParseError(SerdeError),
    ImageReadError(image::ImageError),
    InconsistentVertices,
    NegativeSize,
}

impl StdError for LatticeError {
    fn description(&self) -> &str {
        match self {
            LatticeError::IoError(_) => "There was an error reading your file",
            LatticeError::JsonParseError(_) => "There was a problem parsing json",
            LatticeError::ImageReadError(_) => "There was a problem reading the image",
            LatticeError::InconsistentVertices => "These vertices are inconsistent",
            LatticeError::NegativeSize => "What are you up to don't give me a negative size",
        }
    }

    fn cause(&self) -> Option<&dyn StdError> {
        match self {
            LatticeError::IoError(err) => Some(err),
            LatticeError::JsonParseError(err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for LatticeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LatticeError::IoError(_) => f.write_str("IoError"),
            LatticeError::JsonParseError(_) => f.write_str("JsonParseError"),
            LatticeError::ImageReadError(_) => f.write_str("ImageReadError"),
            LatticeError::InconsistentVertices => f.write_str("InconsistentVertices"),
            LatticeError::NegativeSize => f.write_str("NegativeSize"),
        }
    }
}

impl From<SerdeError> for LatticeError {
    fn from(err: SerdeError) -> Self {
        LatticeError::JsonParseError(err)
    }
}

impl From<IoError> for LatticeError {
    fn from(err: IoError) -> Self {
        LatticeError::IoError(err)
    }
}

impl From<image::ImageError> for LatticeError {
    fn from(err: image::ImageError) -> Self {
        LatticeError::ImageReadError(err)
    }
}
