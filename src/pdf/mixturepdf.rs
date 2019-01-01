extern crate rand;

use tdmath::Vector3;
use pdf::PDF;
use std::f32;
use rand::Rng;

pub struct MixturePDF {
    pdf0: Box<PDF>,
    pdf1: Box<PDF>,
}

impl MixturePDF {
    pub fn new(pdf0: Box<PDF>, pdf1: Box<PDF>) -> MixturePDF {
        MixturePDF {
            pdf0,
            pdf1,
        }
    }
}

impl PDF for MixturePDF {
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