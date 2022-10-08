//! Simpe tools for dealing with masks, and abstracts the requirement of
//! image.

extern crate image;

use std::error::Error;
use std::path::Path;

use self::image::{DynamicImage, GenericImage, Pixel};
use rand::{thread_rng, Rng, ThreadRng};

pub struct Mask {
    image: Box<DynamicImage>,
    /// Pixels per unit
    ppu: f64,
    rng: ThreadRng,
}

impl Mask {
    pub fn new(path: &Path, ppu: f64) -> Result<Self, Box<dyn Error>> {
        let img = image::open(path)?;
        Ok(Self {
            image: Box::new(img),
            ppu,
            rng: thread_rng(),
        })
    }

    pub fn keep(&mut self, x: f64, y: f64) -> bool {
        let i = (x * self.ppu).floor() as u32 % self.image.width();
        let j = (y * self.ppu).floor() as u32 % self.image.height();
        let j = self.image.height() - j - 1;
        let alpha = self.image.get_pixel(i, j).channels()[3];
        let prob = f64::from(alpha) / 255.0;
        let shoot: f64 = self.rng.next_f64();
        shoot < prob
    }
}
