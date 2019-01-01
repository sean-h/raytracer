use material::{Material, ScatterRecord};
use texture::Texture;
use tdmath::{Vector3, Ray};
use hitable::HitRecord;
use std::f32;
use onb::ONB;

pub struct Lambertion {
    albedo: Box<Texture>,
}

impl Lambertion {
    pub fn new(albedo: Box<Texture>) -> Self {
        Lambertion {
            albedo,
        }
    }
}

impl Material for Lambertion {
    fn scatter(&self, ray: Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let uvw = ONB::from_w(hit_record.normal());
        let direction = uvw.local(Vector3::random_cosine_direction());
        let scattered = Ray::new(hit_record.p(), direction.normalized(), ray.time());
        let attenuation = self.albedo.value(hit_record.u(), hit_record.v(), hit_record.p());
        let pdf = Vector3::dot(hit_record.normal(), scattered.direction()) / f32::consts::PI;

        Some(ScatterRecord::new(attenuation, scattered, pdf))
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
