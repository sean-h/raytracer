use tdmath::{Vector3, Ray};
use hitable::*;
use rect::*;
use material::Material;
use aabb::AABB;
use std::sync::Arc;

pub struct Cube {
    min: Vector3,
    max: Vector3,
    faces: Vec<Box<Hitable>>,
}

impl Cube {
    pub fn new(min: Vector3, max: Vector3, material: Arc<Material>) -> Self {
        let mut faces: Vec<Box<Hitable>> = Vec::new();

        faces.push(Box::new(XYRect::new(min.x, max.x, min.y, max.y, max.z, material.clone())));
        faces.push(Box::new(FlipNormals::new(Box::new(XYRect::new(min.x, max.x, min.y, max.y, min.z, material.clone())))));
        faces.push(Box::new(XZRect::new(min.x, max.x, min.z, max.z, max.y, material.clone())));
        faces.push(Box::new(FlipNormals::new(Box::new(XZRect::new(min.x, max.x, min.z, max.z, min.y, material.clone())))));
        faces.push(Box::new(YZRect::new(min.y, max.y, min.z, max.z, max.x, material.clone())));
        faces.push(Box::new(FlipNormals::new(Box::new(YZRect::new(min.y, max.y, min.z, max.z, min.x, material.clone())))));

        Cube {
            min,
            max,
            faces,
        }
    }
}

impl Hitable for Cube {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for hitable in &self.faces {
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

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB::new(self.min, self.max))
    }
}