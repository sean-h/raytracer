use ray::Ray;
use vector3::Vector3;

pub struct HitRecord {
    t: f32,
    p: Vector3,
    normal: Vector3,
}

impl HitRecord {
    pub fn new(t: f32, p: Vector3, normal: Vector3) -> HitRecord {
        HitRecord {
            t,
            p,
            normal,
        }
    }

    pub fn t(&self) -> f32 {
        self.t
    }

    pub fn p(&self) -> Vector3 {
        self.p
    }

    pub fn normal(&self) -> Vector3 {
        self.normal
    }
}

pub trait Hitable {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}