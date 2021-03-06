extern crate tdmath;

use tdmath::Vector3;
use tdmath::Ray;

#[derive(Debug, Copy, Clone)]
pub struct AABB {
    min: Vector3,
    max: Vector3,
}

impl AABB {
    pub fn new(min: Vector3, max: Vector3) -> Self {
        AABB {
            min,
            max,
        }
    }

    pub fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> bool {
        for i in 0..3 {
            let inv_d = 1.0 / r.direction()[i];
            let mut t0 = (self.min[i] - r.origin()[i]) * inv_d;
            let mut t1 = (self.max[i] - r.origin()[i]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            let tmin = if t0 > tmin {
                t0
            } else {
                tmin
            };

            let tmax = if t1 < tmax {
                t1
            } else {
                tmax
            };

            if tmax <= tmin {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
        let x = fmin(box0.min().x, box1.min().x);
        let y = fmin(box0.min().y, box1.min().y);
        let z = fmin(box0.min().z, box1.min().z);
        let small = Vector3::new(x, y, z);

        let x = fmax(box0.max().x, box1.max().x);
        let y = fmax(box0.max().y, box1.max().y);
        let z = fmax(box0.max().z, box1.max().z);
        let big = Vector3::new(x, y, z);

        AABB::new(small, big)
    }

    pub fn min(&self) -> Vector3 {
        self.min
    }

    pub fn max(&self) -> Vector3 {
        self.max
    }
}

fn fmin(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

fn fmax(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}