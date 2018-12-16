extern crate tdmath;

use tdmath::Ray;
use tdmath::Vector3;
use material::Material;
use aabb::AABB;

pub struct HitRecord<'a> {
    t: f32,
    pub p: Vector3,
    u: f32,
    v: f32,
    pub normal: Vector3,
    material: &'a Box<Material>,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f32, p: Vector3, u: f32, v: f32, normal: Vector3, material: &Box<Material>) -> HitRecord {
        HitRecord {
            t,
            p,
            u,
            v,
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

    pub fn u(&self) -> f32 {
        self.u
    }

    pub fn v(&self) -> f32 {
        self.v
    }

    pub fn normal(&self) -> Vector3 {
        self.normal
    }

    pub fn material(&self) -> &Box<Material> {
        self.material
    }

    pub fn flip_normal(&mut self) {
        self.normal = -self.normal
    }

    pub fn translate(&mut self, offset: Vector3) {
        self.p = self.p + offset;
    }
}

pub trait Hitable {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}

pub struct FlipNormals {
    hitable: Box<Hitable>
}

impl FlipNormals {
    pub fn new(hitable: Box<Hitable>) -> FlipNormals {
        FlipNormals {
            hitable,
        }
    }
}

impl Hitable for FlipNormals {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self.hitable.hit(ray, t_min, t_max) {
            Some(mut hit) => {
                hit.flip_normal();
                Some(hit)
            },
            None => None
        }
    }
    
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.hitable.bounding_box(t0, t1)
    }
}