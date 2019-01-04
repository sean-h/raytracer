use material::Material;
use texture::Texture;
use tdmath::{Vector3, Ray};
use hitable::HitRecord;

pub struct DiffuseLight {
    emit: Box<Texture>
}

impl DiffuseLight {
    pub fn new(emit: Box<Texture>) -> Self {
        DiffuseLight {
            emit,
        }
    }
}

impl Material for DiffuseLight {
    fn emit(&self, ray: Ray, hit: &HitRecord, u: f32, v: f32, p: Vector3) -> Vector3 {
        if Vector3::dot(hit.normal(), ray.direction()) < 0.0 {
            self.emit.value(u, v, p)
        } else {
            Vector3::zero()
        }
    }

    fn sample(&self) -> bool {
        true
    }
}