extern crate rand;

use material::{Material, ScatterRecord};
use tdmath::{Vector3, Ray};
use hitable::HitRecord;
use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub struct Dielectric {
    ref_index: f32,
}

impl Dielectric {
    pub fn new(ref_index: f32) -> Self {
        Dielectric {
            ref_index,
        }
    }

    fn refract(v: Vector3, n: Vector3, ni_over_nt: f32) -> Option<Vector3> {
        let uv = v.normalized();
        let dt = Vector3::dot(uv, n);
        let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

        if discriminant > 0.0 {
            let refracted = (uv - n * dt) * ni_over_nt - n * discriminant.sqrt();
            Some(refracted)
        } else {
            None
        }
    }

    fn schlick(&self, cosine: f32) -> f32 {
        let r0 = (1.0 - self.ref_index) / (1.0 + self.ref_index);
        let r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {        
        let reflected = Vector3::reflect(ray.direction(), hit_record.normal());
        let attenuation = Vector3::new(1.0, 1.0, 1.0);

        let (outward_normal, ni_over_nt, cosine) = if Vector3::dot(ray.direction(), hit_record.normal()) > 0.0 {
            (-hit_record.normal(),
             self.ref_index,
             self.ref_index * Vector3::dot(ray.direction(), hit_record.normal()) / ray.direction().length())
        } else {
            (hit_record.normal(),
             1.0 / self.ref_index,
             -Vector3::dot(ray.direction(), hit_record.normal()) / ray.direction().length())
        };

        let mut refracted = Vector3::zero();

        let reflect_prob = match Dielectric::refract(ray.direction(), outward_normal, ni_over_nt) {
            Some(refract) => {
                refracted = refract;
                self.schlick(cosine)
            },
            None => 1.0,
        };

        let mut rng = rand::thread_rng();
        let scattered = if rng.gen::<f32>() < reflect_prob {
            Ray::new(hit_record.p(), reflected, ray.time())
        } else {
            Ray::new(hit_record.p(), refracted, ray.time())
        };

        return Some(ScatterRecord::new(attenuation, None, None));
    }
}