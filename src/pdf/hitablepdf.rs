use tdmath::Vector3;
use pdf::*;
use std::f32;
use hitable::Hitable;

pub struct HitablePDF<'a> {
    origin: Vector3,
    hitable: &'a Hitable,
}

impl<'a> HitablePDF<'a> {
    pub fn new(origin: Vector3, hitable: &'a Hitable) -> HitablePDF {
        HitablePDF {
            origin,
            hitable,
        }
    }
}

impl<'a> PDF for HitablePDF<'a> {
    fn value(&self, direction: Vector3) -> f32 {
        self.hitable.pdf_value(self.origin, direction)
    }

    fn generate(&self) -> Vector3 {
        self.hitable.random(self.origin)
    }
}
