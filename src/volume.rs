extern crate rand;

use hitable::{Hitable, HitRecord};
use material::{Material, ScatterRecord};
use texture::Texture;
use tdmath::{Vector3, Ray};
use aabb::AABB;
use rand::Rng;
use std::f32;

pub struct ConstantMedium {
    boundary: Box<Hitable>,
    density: f32,
    phase_function: Box<Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Box<Hitable>, density: f32, texture: Box<Texture>) -> Self {
        ConstantMedium {
            boundary,
            density,
            phase_function: Box::new(Isotropic::new(texture)),
        }        
    }
}

impl Hitable for ConstantMedium {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self.boundary.hit(ray, -f32::MAX, f32::MAX) {
            Some(mut hit1) => {
                match self.boundary.hit(ray, hit1.t() + 0.0001, f32::MAX) {
                    Some(mut hit2) => {
                        if hit1.t < t_min {
                            hit1.t = t_min;
                        }
                        if hit2.t > t_max {
                            hit2.t = t_max;
                        }
                        if hit1.t >= hit2.t {
                            return None;
                        }

                        if hit1.t < 0.0 {
                            hit1.t = 0.0;
                        }

                        let distance_inside_boundary = (hit2.t - hit1.t) * ray.direction().length();
                        let mut rng = rand::thread_rng();
                        let hit_distance = -(1.0 / self.density) * rng.gen::<f32>().ln();
                        if hit_distance < distance_inside_boundary {
                            let t = hit1.t + hit_distance / ray.direction().length();
                            let p = ray.point_at_parameter(t);
                            let normal = Vector3::new(1.0, 0.0, 0.0);

                            Some(HitRecord::new(t, p, hit1.u(), hit1.v(), normal, &self.phase_function))
                        } else {
                            None
                        }
                    },
                    None => None
                }
            },
            None => None
        }
    }
    
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.boundary.bounding_box(t0, t1)
    }
}

struct Isotropic {
    albedo: Box<Texture>,
}

impl Isotropic {
    pub fn new(albedo: Box<Texture>) -> Self {
        Isotropic {
            albedo
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, _ray: Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let scattered = Ray::new(hit_record.p(), Vector3::random_in_unit_sphere(), 0.0);
        let attenuation = self.albedo.value(hit_record.u(), hit_record.v(), hit_record.p());

        Some(ScatterRecord::new(attenuation, scattered))
    }

    fn emit(&self, u: f32, v: f32, p: Vector3) -> Vector3 {
        Vector3::zero()
    }
}