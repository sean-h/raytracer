use material::{Material, ScatterRecord};
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
    fn scatter(&self, ray: Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn emit(&self, u: f32, v: f32, p: Vector3) -> Vector3 {
        self.emit.value(u, v, p)
    }
}