//! Simpe tools for dealing with masks, and abstracts the requirement of
//! image.

use crate::error::VegasLatticeError;
use image::{DynamicImage, GenericImageView, Pixel};
use rand::Rng;
use std::path::Path;

/// A mask represents a 2D image that can be used to filter out points
///
/// # Example
///
/// ```rust
/// use vegas_lattice::Mask;
/// use std::path::Path;
///
/// let mask = Mask::try_new(Path::new("docs/bpm.png"), 1.0).unwrap();
/// let keep = mask.keep(0.5, 0.5, &mut rand::rng());
///
/// assert!(keep || !keep);
/// ```
pub struct Mask {
    /// The image
    image: Box<DynamicImage>,
    /// Pixels per unit
    ppu: f64,
}

impl Mask {
    /// Create a new mask from a path and a pixels per unit ratio
    pub fn try_new(path: &Path, ppu: f64) -> Result<Self, VegasLatticeError> {
        let img = image::open(path)?;
        Ok(Self {
            image: Box::new(img),
            ppu,
        })
    }

    /// Computes wheter to keep a site or not given the coordinates of the site and a random
    /// number generator.
    pub fn keep<R: Rng + ?Sized>(&self, x: f64, y: f64, rng: &mut R) -> bool {
        // TODO: Consider using python_mod here
        let i = (x * self.ppu).floor() as u32 % self.image.width();
        let j = (y * self.ppu).floor() as u32 % self.image.height();
        let j = self.image.height() - j - 1;
        let alpha = self.image.get_pixel(i, j).channels()[3];
        let prob = f64::from(alpha) / 255.0;
        let shoot: f64 = rng.random();
        shoot < prob
    }
}
