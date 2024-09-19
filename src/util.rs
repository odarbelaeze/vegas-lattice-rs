//! General utilities that have nothing to do with lattices

/// Represents a 3D axis
#[derive(Debug, Clone, Copy)]
pub enum Axis {
    /// The _x_ axis
    X,
    /// The _y_ axis
    Y,
    /// The _z_ axis
    Z,
}

pub fn python_mod(num: i32, modulus: usize) -> (i32, i32) {
    if num < 0 {
        (
            modulus as i32 + num % modulus as i32,
            num / modulus as i32 - 1,
        )
    } else {
        (num % modulus as i32, num / modulus as i32)
    }
}

/// A trait for tagged objects
pub trait Tagged {
    /// Returns the tags of the object
    fn tags(&self) -> Option<&Vec<String>>;
    /// Returns `true` if the object has the given tag
    fn has_tag(&self, tag: &str) -> bool {
        match self.tags() {
            None => false,
            Some(tags) => tags.iter().any(|t| t == tag),
        }
    }
}
