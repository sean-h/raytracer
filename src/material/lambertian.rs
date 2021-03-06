use material::{Material, ScatterRecord, ScatterType};
use texture::Texture;
use tdmath::{Vector3, Ray};
use hitable::HitRecord;
use std::f32;
use pdf::CosinePDF;

pub struct Lambertian {
    albedo: Box<Texture>,
}

impl Lambertian {
    pub fn new(albedo: Box<Texture>) -> Self {
        Lambertian {
            albedo,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = self.albedo.value(hit_record.u(), hit_record.v(), hit_record.p());
        let pdf = CosinePDF::new(hit_record.normal());
        let scatter_type = ScatterType::Scatter(Box::new(pdf));

        Some(ScatterRecord::new(attenuation, scatter_type))
    }

    fn scattering_pdf(&self, _ray: Ray, hit_record: &HitRecord, scattered: Ray) -> f32 {
        let cos = Vector3::dot(hit_record.normal(), scattered.direction().normalized());
        if cos < 0.0 {
            0.0
        } else {
            cos / f32::consts::PI
        }
    }
}
