extern crate rand;
extern crate tdmath;

use hitable::*;
use aabb::AABB;
use tdmath::Ray;
use rand::Rng;

pub struct BVH {
    left: Option<Box<Hitable>>,
    right: Option<Box<Hitable>>,
    bbox: AABB,
}

impl BVH {
    pub fn from_list(mut hitables: Vec<Box<Hitable>>, t0: f32, t1: f32) -> Self {
        hitables = BVH::sort_list_by_random_axis(hitables, t0, t1);

        if hitables.len() == 1 {
            let left = hitables.remove(0);
            let bbox = left.bounding_box(t0, t1).unwrap();

            return BVH {
                left: Some(left),
                right: None,
                bbox
            }
        } else if hitables.len() == 2 {
            let left = hitables.remove(0);
            let right = hitables.remove(0);
            let bbox = AABB::surrounding_box(left.bounding_box(t0, t1).unwrap(), right.bounding_box(t0, t1).unwrap());

            return BVH {
                left: Some(left),
                right: Some(right),
                bbox,
            };
        } else {
            let n = hitables.len();
            let mut left_list = hitables;
            let right_list = left_list.split_off(n / 2);

            let left = BVH::from_list(left_list, t0, t1);
            let right = BVH::from_list(right_list, t0, t1);
            let bbox = AABB::surrounding_box(left.bounding_box(t0, t1).unwrap(), right.bounding_box(t0, t1).unwrap());

            return BVH {
                left: Some(Box::new(left)),
                right: Some(Box::new(right)),
                bbox,
            };
        }
    }

    fn sort_list_by_random_axis(mut list: Vec<Box<Hitable>>, t0: f32, t1: f32) -> Vec<Box<Hitable>> {
        let mut rng = rand::thread_rng();
        match (rng.gen::<f32>() * 3.0) as i32 {
            0 => {
                list.sort_by(|a, b| {
                    if let (Some(box_a), Some(box_b)) = (a.bounding_box(t0, t1), b.bounding_box(t0, t1)) {
                        (box_a.min().x as i32).cmp(&(box_b.min().x as i32))
                    } else {
                        panic!("Bounding box not created for hitable")
                    }
                })
            },
            1 => {
                list.sort_by(|a, b| {
                    if let (Some(box_a), Some(box_b)) = (a.bounding_box(t0, t1), b.bounding_box(t0, t1)) {
                        (box_a.min().y as i32).cmp(&(box_b.min().y as i32))
                    } else {
                        panic!("Bounding box not created for hitable")
                    }
                })
            },
            _ => {
                list.sort_by(|a, b| {
                    if let (Some(box_a), Some(box_b)) = (a.bounding_box(t0, t1), b.bounding_box(t0, t1)) {
                        (box_a.min().z as i32).cmp(&(box_b.min().z as i32))
                    } else {
                        panic!("Bounding box not created for hitable")
                    }
                })
            }
        }

        list
    }
}

impl Hitable for BVH {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if self.bbox.hit(&ray, t_min, t_max) {
            let hit_left = if let Some(l) = &self.left {
                l.hit(ray, t_min, t_max)
            } else {
                None
            };

            let hit_right = if let Some(r) = &self.right {
                r.hit(ray, t_min, t_max)
            } else {
                None
            };

            match hit_left {
                Some(l) => {
                    match hit_right {
                        Some(r) => {
                            if l.t() < r.t() {
                                return Some(l);
                            } else {
                                return Some(r);
                            }
                        },
                        None => return Some(l),
                    }
                }
                None => {
                    match hit_right {
                        Some(r) => return Some(r),
                        None => return None,
                    }
                }
            }
        } else {
            return None;
        }
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(self.bbox)
    }
}