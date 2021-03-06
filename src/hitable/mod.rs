extern crate tdmath;
extern crate rand;

pub mod cube;
pub mod rect;
pub mod sphere;
pub mod volume;
pub mod triangle;
pub mod mesh;

pub use self::cube::Cube;
pub use self::rect::{XYRect, XZRect, YZRect};
pub use self::sphere::Sphere;
pub use self::triangle::Triangle;
pub use self::mesh::Mesh;

use tdmath::Ray;
use tdmath::Vector3;
use material::Material;
use aabb::AABB;

pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vector3,
    u: f32,
    v: f32,
    pub normal: Vector3,
    pub material: &'a Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f32, p: Vector3, u: f32, v: f32, normal: Vector3, material: &'a Material) -> HitRecord {
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

    pub fn flip_normal(&mut self) {
        self.normal = -self.normal
    }

    pub fn translate(&mut self, offset: Vector3) {
        self.p = self.p + offset;
    }
}

pub trait Hitable: Send + Sync {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;

    fn pdf_value(&self, _origin: Vector3, _v: Vector3) -> f32 {
        0.0
    }

    fn random(&self, _origin: Vector3) -> Vector3 {
        Vector3::new(1.0, 0.0, 0.0)
    }
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

    fn pdf_value(&self, origin: Vector3, v: Vector3) -> f32 {
        self.hitable.pdf_value(origin, v)
    }

    fn random(&self, origin: Vector3) -> Vector3 {
        self.hitable.random(origin)
    }
}
