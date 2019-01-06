extern crate rand;

use tdmath::Vector3;
use pdf::*;
use std::f32;
use rand::Rng;

pub struct MixturePDF<'a> {
    pdf0: &'a PDF,
    pdf1: &'a PDF,
}

impl<'a> MixturePDF<'a> {
    pub fn new(pdf0: &'a PDF, pdf1: &'a PDF) -> MixturePDF<'a> {
        MixturePDF {
            pdf0,
            pdf1,
        }
    }
}

impl<'a> PDF for MixturePDF<'a> {
    fn value(&self, direction: Vector3) -> f32 {
        0.5 * self.pdf0.value(direction) + 0.5 * self.pdf1.value(direction)
    }

    fn generate(&self) -> Vector3 {
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < 0.5 {
            self.pdf0.generate()
        } else {
            self.pdf1.generate()
        }
    }
}