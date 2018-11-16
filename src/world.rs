use hitable::*;
use ray::Ray;
use aabb::AABB;

pub struct World {
    hitables: Vec<Box<Hitable>>,
}

impl World {
    pub fn new() -> World {
        World {
            hitables: Vec::new(),
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