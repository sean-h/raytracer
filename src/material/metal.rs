use material::{Material, ScatterRecord, ScatterType};
use tdmath::{Vector3, Ray};
use hitable::HitRecord;

#[derive(Debug, Copy, Clone)]
pub struct Metal {
    albedo: Vector3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vector3, fuzz: f32) -> Self {
        let fuzz = if fuzz < 1.0 {
            fuzz
        } else {
            1.0
        };

        Metal {
            albedo,
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let reflected = Vector3::reflect(ray.direction().normalized(), hit_record.normal());
        let scattered = Ray::new(hit_record.p(), reflected + Vector3::random_in_unit_sphere() * self.fuzz, ray.time());
        let scatter_type = ScatterType::Specular(scattered);

        Some(ScatterRecord::new(self.albedo, scatter_type))
    }
}