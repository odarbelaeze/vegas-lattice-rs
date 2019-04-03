use std::error::Error as StdError;
use std::fmt;

use serde_json::Error as SerdeError;

#[derive(Debug)]
pub enum LatticeError {
    JsonParseError(SerdeError),
    InconsistentVertices,
    NegativeSize,
}

impl fmt::Display for LatticeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LatticeError::JsonParseError(_) => f.write_str("JsonParseError"),
            LatticeError::InconsistentVertices => f.write_str("InconsistentVertices"),
            LatticeError::NegativeSize => f.write_str("NegativeSize"),
        }
    }
}

impl StdError for LatticeError {
    fn description(&self) -> &str {
        match *self {
            LatticeError::JsonParseError(_) => "failed to parse JSON",
            LatticeError::InconsistentVertices => "the vertices are inconsistent",
            LatticeError::NegativeSize => "a negative size value is hard to grasp",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            LatticeError::JsonParseError(ref err) => Some(err),
            _ => None,
        }
    }
}
