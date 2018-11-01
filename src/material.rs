use ray::Ray;
use hitable::HitRecord;
use vector3::Vector3;

pub struct ScatterRecord {
    attenuation: Vector3,
    scattered: Ray,
}

impl ScatterRecord {
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