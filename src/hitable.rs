use ray::Ray;
use vector3::Vector3;
use material::Material;

pub struct HitRecord<'a> {
    t: f32,
    p: Vector3,
    normal: Vector3,
    material: &'a Box<Material>,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f32, p: Vector3, normal: Vector3, material: &Box<Material>) -> HitRecord {
        HitRecord {
            t,
            p,
            normal,
            material,
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

    pub fn material(&self) -> &Box<Material> {
        self.material
    }
}

pub trait Hitable {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}