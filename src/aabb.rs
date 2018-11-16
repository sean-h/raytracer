use vector3::Vector3;
use ray::Ray;

#[derive(Debug, Copy, Clone)]
pub struct AABB {
    min: Vector3,
    max: Vector3,
}

impl AABB {
    pub fn new(min: Vector3, max: Vector3) -> AABB {
        AABB {
            min,
            max,
        }
    }

    // TODO: Replace with optimized version
    pub fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> bool {
        for i in 0..3 {
            let a = (self.min[i] - r.origin()[i]) / r.direction()[i];
            let b = (self.max[i] - r.origin()[i]) / r.direction()[i];

            let t0 = fmin(a, b);
            let t1 = fmax(a, b);
            let tmin = fmax(t0, tmin);
            let tmax = fmin(t1, tmax);
            if tmax <= tmin {
                return false
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