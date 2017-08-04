//! Simpe tools for dealing with masks, and abstracts the requirement of
//! image.


use std::error::Error;
use std::path::Path;

use image::{self, DynamicImage, GenericImage, Pixel};
use rand::{thread_rng, ThreadRng, Rng};

/// Abstracts the concept of a mask
pub struct Mask {
    /// Points to a generic image to draw data from
    image: Box<DynamicImage>,
    /// Pixels per unit
    ppu: f64,
    /// Random number generator
    rng: ThreadRng,
}


impl Mask {
    // TODO This maybe a path
    pub fn new(path: &Path, ppu: f64) -> Result<Self, Box<Error>> {
        let img = image::open(path)?;
        Ok(Self {
               image: Box::new(img),
               ppu: ppu,
               rng: thread_rng(),
           })
    }

    pub fn keep(&mut self, x: f64, y: f64) -> bool {
        let i = (x * self.ppu).floor() as u32 % self.image.width();
        let j = (y * self.ppu).floor() as u32 % self.image.height();
        let alpha = self.image.get_pixel(i, j).channels()[3];
        let prob = alpha as f64 / 255.0;
        let shoot: f64 = self.rng.next_f64();
        shoot < prob        
    }
}