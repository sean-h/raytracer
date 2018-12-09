extern crate tdmath;
extern crate toml;

use hitable::*;
use tdmath::{Ray, Vector3};
use aabb::AABB;
use self::toml::Value;
use sphere::Sphere;
use material::Lambertion;
use texture::ConstantTexture;

pub struct World {
    hitables: Vec<Box<Hitable>>,
}

impl World {
    pub fn new() -> World {
        World {
            hitables: Vec::new(),
        }
    }

    pub fn from_toml(scene: &Value) -> World {
        let mut hitables: Vec<Box<Hitable>> = Vec::new();

        let objects = scene["objects"].as_table().unwrap();

        for (obj_name, obj_data) in objects.iter() {
            let obj_type = obj_data["type"].as_str().unwrap();
            let position = obj_data["position"].as_array().unwrap();
            let x = position[0].as_float().unwrap() as f32;
            let y = position[1].as_float().unwrap() as f32;
            let z = position[2].as_float().unwrap() as f32;
            let radius = obj_data["radius"].as_float().unwrap() as f32;
            let material_name = obj_data["material"].as_str().unwrap();

            let texture = ConstantTexture::new(Vector3::new(0.7, 0.7, 0.7));
            let material = Lambertion::new(Box::new(texture));
            let sphere = Sphere::new(Vector3::new(x, y, z), radius, Box::new(material));
            hitables.push(Box::new(sphere));
        }

        World {
            hitables,
        }
    }

    pub fn add_hitable(&mut self, hitable: Box<Hitable>) {
        self.hitables.push(hitable);
    }
}

impl Hitable for World {
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
}