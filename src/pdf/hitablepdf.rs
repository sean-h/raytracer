use tdmath::Vector3;
use pdf::PDF;
use std::f32;
use hitable::Hitable;
use std::sync::Arc;

pub struct HitablePDF {
    origin: Vector3,
    hitable: Arc<Hitable>,
}

impl HitablePDF {
    pub fn new(origin: Vector3, hitable: Arc<Hitable>) -> HitablePDF {
        HitablePDF {
            origin,
            hitable,
        }
    }
}

impl PDF for HitablePDF {
    fn value(&self, direction: Vector3) -> f32 {
        self.hitable.pdf_value(self.origin, direction)
    }

    fn generate(&self) -> Vector3 {
        self.hitable.random(self.origin)
    }
}