extern crate tdmath;

use hitable::{Hitable, HitRecord};
use material::Material;
use aabb::AABB;
use tdmath::{Vector3, Ray};

pub struct XYRect {
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
    material: Box<Material>,
}

impl XYRect {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: Box<Material>) -> XYRect {
        XYRect {
            x0,
            x1,
            y0,
            y1,
            k,
            material,
        }
    }
}

impl Hitable for XYRect {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin().z) / ray.direction().z;
        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin().x + t * ray.direction().x;
        let y = ray.origin().y + t * ray.direction().y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        let p = ray.point_at_parameter(t);

        
        
        Some(HitRecord::new(t, p, u, v, Vector3::new(0.0, 0.0, 1.0), &self.material))
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB::new(Vector3::new(self.x0, self.y0, self.k - 0.0001), Vector3::new(self.x1, self.y1, self.k + 0.0001)))
    }
}

pub struct XZRect {
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    k: f32,
    material: Box<Material>,
}

impl XZRect {
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, material: Box<Material>) -> XZRect {
        XZRect {
            x0,
            x1,
            z0,
            z1,
            k,
            material,
        }
    }
}

impl Hitable for XZRect {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin().y) / ray.direction().y;
        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin().x + t * ray.direction().x;
        let z = ray.origin().z + t * ray.direction().z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let p = ray.point_at_parameter(t);

        Some(HitRecord::new(t, p, u, v, Vector3::new(0.0, 1.0, 0.0), &self.material))
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB::new(Vector3::new(self.x0, self.k - 0.0001, self.z0), Vector3::new(self.x1, self.k + 0.0001, self.z1)))
    }
}

pub struct YZRect {
    y0: f32,
    y1: f32,
    z0: f32,
    z1: f32,
    k: f32,
    material: Box<Material>,
}

impl YZRect {
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, material: Box<Material>) -> YZRect {
        YZRect {
            y0,
            y1,
            z0,
            z1,
            k,
            material,
        }      
    }
}

impl Hitable for YZRect {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin().x) / ray.direction().x;
        if t < t_min || t > t_max {
            return None;
        }

        let y = ray.origin().y + t * ray.direction().y;
        let z = ray.origin().z + t * ray.direction().z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let p = ray.point_at_parameter(t);

        Some(HitRecord::new(t, p, u, v, Vector3::new(1.0, 0.0, 0.0), &self.material))
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB::new(Vector3::new(self.k - 0.0001, self.y0, self.z0), Vector3::new(self.k + 0.0001, self.y1, self.z1)))
    }
}