//! General utilities that have nothing to do with lattices

#[derive(Debug, Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub fn map(prefix: Option<String>) -> Vec<(String, Axis)> {
        let axes = vec![("x", Axis::X), ("y", Axis::Y), ("z", Axis::Z)];
        match prefix {
            Some(p) => axes
                .into_iter()
                .map(|(k, i)| (format!("{}{}", p, k), i))
                .collect(),
            None => axes.into_iter().map(|(k, i)| (k.to_string(), i)).collect(),
        }
    }
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

pub trait Tagged {
    fn tags(&self) -> Option<&Vec<String>>;
    fn has_tag(&self, tag: String) -> bool {
        match self.tags() {
            None => false,
            Some(ref tags) => tags.iter().any(|t| t == &tag),
        }
    }
}
