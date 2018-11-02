extern crate rand;

use ray::Ray;
use hitable::HitRecord;
use vector3::Vector3;
use rand::Rng;

pub struct ScatterRecord {
    attenuation: Vector3,
    scattered: Ray,
}

impl ScatterRecord {
    pub fn new(attenuation: Vector3, scattered: Ray) -> ScatterRecord {
        ScatterRecord {
            attenuation,
            scattered,
        }
    }

    pub fn attenuation(&self) -> Vector3 {
        self.attenuation
    }

    pub fn scattered(&self) -> Ray {
        self.scattered
    }
}

pub trait Material {
    fn scatter(&self, ray: Ray, hit_record: &HitRecord) -> Option<ScatterRecord>;
}

#[derive(Debug, Copy, Clone)]
pub struct Lambertion {
    albedo: Vector3,
}

impl Lambertion {
    pub fn new(albedo: Vector3) -> Lambertion {
        Lambertion {
            albedo,
        }
    }
}

impl Material for Lambertion {
    fn scatter(&self, ray: Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let target = hit_record.p() + hit_record.normal() + Vector3::random_in_unit_sphere();

        Some(ScatterRecord {
            scattered: Ray::new(hit_record.p(), target - hit_record.p()),
            attenuation: self.albedo,
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Metal {
    albedo: Vector3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vector3, fuzz: f32) -> Metal {
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
        let scattered = Ray::new(hit_record.p(), reflected + Vector3::random_in_unit_sphere() * self.fuzz);

        Some(ScatterRecord {
            scattered,
            attenuation: self.albedo,
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Dielectric {
    ref_index: f32,
}

impl Dielectric {
    pub fn new(ref_index: f32) -> Dielectric {
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
            Ray::new(hit_record.p(), reflected)
        } else {
            Ray::new(hit_record.p(), refracted)
        };

        return Some(ScatterRecord::new(attenuation, scattered));
    }
}