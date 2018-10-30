use hitable::*;
use ray::Ray;

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
}