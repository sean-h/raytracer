use tdmath::{Vector3, Ray};
use hitable::{Hitable, HitRecord};
use material::Material;
use aabb::AABB;

pub struct Triangle {
    v0: Vector3,
    v1: Vector3,
    v2: Vector3,
    material: Box<Material>,
}

impl Triangle {
    pub fn new(v0: Vector3, v1: Vector3, v2: Vector3, material: Box<Material>) -> Self {
        Triangle {
            v0,
            v1,
            v2,
            material,
        }
    }
}

impl Hitable for Triangle {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let e1 = self.v1 - self.v0;
        let e2 = self.v2 - self.v0;
        let normal = Vector3::cross(e1, e2).normalized();

        if Vector3::dot(normal, ray.direction()) > 0.0 {
            return None;
        }

        let h = Vector3::cross(ray.direction(), e2);
        let a = Vector3::dot(e1, h);
        if a > -std::f32::EPSILON && a < std::f32::EPSILON {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin() - self.v0;
        let u = f * Vector3::dot(s, h);
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = Vector3::cross(s, e1);
        let v = f * Vector3::dot(ray.direction(), q);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * Vector3::dot(e2, q);

        if t > t_min && t < t_max {
            let p = (1.0 - u - v) * self.v0 + u * self.v1 + v * self.v2;
            Some(HitRecord::new(t, p, u, v, normal, &*self.material))
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        None
    }
}