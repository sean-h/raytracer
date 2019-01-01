use onb::ONB;
use tdmath::Vector3;
use pdf::PDF;
use std::f32;

pub struct CosinePDF {
    uvw: ONB,
}

impl CosinePDF {
    pub fn new(w: Vector3) -> CosinePDF {
        CosinePDF {
            uvw: ONB::from_w(w),
        }
    }
}

impl PDF for CosinePDF {
    fn value(&self, direction: Vector3) -> f32 {
        let cosine = Vector3::dot(direction.normalized(), self.uvw.w());
        if cosine > 0.0 {
            cosine / f32::consts::PI
        } else {
            0.0
        }
    }

    fn generate(&self) -> Vector3 {
        self.uvw.local(Vector3::random_cosine_direction())
    }
}