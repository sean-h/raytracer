extern crate tdmath;
extern crate rand;

use tdmath::Ray;
use tdmath::Vector3;
use material::Material;
use aabb::AABB;
use std::sync::Arc;
use rand::Rng;

pub struct HitRecord {
    pub t: f32,
    pub p: Vector3,
    u: f32,
    v: f32,
    pub normal: Vector3,
    material: Arc<Material>,
}

impl HitRecord {
    pub fn new(t: f32, p: Vector3, u: f32, v: f32, normal: Vector3, material: Arc<Material>) -> HitRecord {
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

    pub fn material(&self) -> Arc<Material> {
        self.material.clone()
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

    fn pdf_value(&self, origin: Vector3, v: Vector3) -> f32 {
        0.0
    }

    fn random(&self, origin: Vector3) -> Vector3 {
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
}

pub struct HitableList {
    hitables: Vec<Arc<Hitable>>,
}

impl HitableList {
    pub fn new(hitables: Vec<Arc<Hitable>>) -> HitableList {
        HitableList {
            hitables,
        }
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for hitable in &self.hitables {
            match hitable.hit(ray, t_min, closest_so_far) {
                Some(hit) => {
                    closest_so_far = hit.t();
                    hit_record = Some(hit);
                },
                None => ()
            }
        }

        hit_record
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if self.hitables.len() == 0 {
            return None;
        }

        let mut bbox = match self.hitables[0].bounding_box(t0, t1) {
            Some(aabb) => aabb,
            None => return None,
        };

        for hitable in &self.hitables {
            let aabb = match hitable.bounding_box(t0, t1) {
                Some(aabb) => aabb,
                None => return None,
            };

            bbox = AABB::surrounding_box(bbox, aabb);
        }

        Some(bbox)
    }

    fn pdf_value(&self, origin: Vector3, v: Vector3) -> f32 {
        let weight = 1.0 / self.hitables.len() as f32;
        let mut sum = 0.0;
        for hitable in &self.hitables {
            sum += weight * hitable.pdf_value(origin, v);
        }

        sum
    }

    fn random(&self, origin: Vector3) -> Vector3 {
        let mut rng = rand::thread_rng();
        let index = (rng.gen::<f32>() * self.hitables.len() as f32) as usize;

        self.hitables[index].random(origin)
    }
}