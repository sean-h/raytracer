use material::{Material, ScatterRecord};
use texture::Texture;
use tdmath::{Vector3, Ray};
use hitable::HitRecord;

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
        let target = hit_record.p() + hit_record.normal() + Vector3::random_in_unit_sphere();
        let attenuation = self.albedo.value(hit_record.u(), hit_record.v(), hit_record.p());
        let scattered = Ray::new(hit_record.p(), target - hit_record.p(), ray.time());

        Some(ScatterRecord::new(attenuation, scattered))
    }
}
