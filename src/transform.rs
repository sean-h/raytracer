use hitable::{Hitable, HitRecord};
use tdmath::{Vector3, Ray};
use aabb::AABB;
use std::f32;

pub struct Translate {
    hitable: Box<Hitable>,
    offset: Vector3,
}

impl Translate {
    pub fn new(hitable: Box<Hitable>, offset: Vector3) -> Translate {
        Translate {
            hitable,
            offset,
        }
    }
}

impl Hitable for Translate {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved = Ray::new(ray.origin() - self.offset, ray.direction(), ray.time());
        match self.hitable.hit(moved, t_min, t_max) {
            Some(mut hit) => {
                hit.translate(self.offset);
                Some(hit)
            },
            None => None,
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        match self.hitable.bounding_box(t0, t1) {
            Some(aabb) => {
                Some(AABB::new(aabb.min() + self.offset, aabb.max() + self.offset))
            },
            None => None
        }
    }
}

pub struct RotateY {
    hitable: Box<Hitable>,
    sin_theta: f32,
    cos_theta: f32,
    bbox: Option<AABB>,
}

impl RotateY {
    pub fn new(hitable: Box<Hitable>, angle: f32) -> RotateY {
        let radians = f32::consts::PI / 180.0 * angle;
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = match hitable.bounding_box(0.0, 1.0) {
            Some(bbox) => {
                let mut min = Vector3::new(f32::MAX, f32::MAX, f32::MAX);
                let mut max = Vector3::new(-f32::MAX, -f32::MAX, -f32::MAX);

                for i in 0..2 {
                    for j in 0..2 {
                        for k in 0..2 {
                            let x = i as f32 * bbox.max().x + (1 - i) as f32 * bbox.min().x;
                            let y = j as f32 * bbox.max().y + (1 - j) as f32 * bbox.min().y;
                            let z = k as f32 * bbox.max().x + (1 - k) as f32 * bbox.min().z;

                            let newx = cos_theta * x + sin_theta * z;
                            let newz = -sin_theta * x + cos_theta * z;
                            let tester = Vector3::new(newx, y, newz);
                            for c in 0..3 {
                                if tester[c] > max[c] {
                                    max[c] = tester[c];
                                }
                                if tester[c] < min[c] {
                                    min[c] = tester[c];
                                }
                            }
                        }
                    }
                }

                Some(AABB::new(min, max))
            },
            None => None
        };

        RotateY {
            hitable,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hitable for RotateY {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut origin = ray.origin();
        let mut direction = ray.direction();

        origin[0] = self.cos_theta * ray.origin()[0] - self.sin_theta * ray.origin()[2];
        origin[2] = self.sin_theta * ray.origin()[0] + self.cos_theta * ray.origin()[2];

        direction[0] = self.cos_theta * ray.direction()[0] - self.sin_theta * ray.direction()[2];
        direction[2] = self.sin_theta * ray.direction()[0] + self.cos_theta * ray.direction()[2];

        let rotated_ray = Ray::new(origin, direction, ray.time());

        match self.hitable.hit(rotated_ray, t_min, t_max) {
            Some(mut hit) => {
                let mut p = hit.p();
                let mut normal = hit.normal();
                p[0] = self.cos_theta * hit.p()[0] + self.sin_theta * hit.p()[2];
                p[2] = -self.sin_theta * hit.p()[0] + self.cos_theta * hit.p()[2];

                normal[0] = self.cos_theta * hit.normal()[0] + self.sin_theta * hit.normal()[2];
                normal[2] = -self.sin_theta * hit.normal()[0] + self.cos_theta * hit.normal()[2];

                hit.p = p;
                hit.normal = normal;
                Some(hit)
            },
            None => None
        }
    }
    
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.bbox
    }
}