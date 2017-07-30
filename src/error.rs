use std::fmt;

use serde_json::Error as SerdeError;

use std::error::Error as StdError;


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
            LatticeError::JsonParseError(_) => "Failed to parse JSON.",
            LatticeError::InconsistentVertices => "The vertices are inconsistent.",
            LatticeError::NegativeSize => "A negative size value is hard to grasp.",
        }
    }
}