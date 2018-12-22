extern crate tdmath;

use hitable::*;
use tdmath::Vector3;
use tdmath::Ray;
use material::Material;
use aabb::AABB;
use std::f32;

pub struct Sphere {
    center: Vector3,
    radius: f32,
    material: Box<Material>,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f32, material: Box<Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn get_sphere_uv(p: Vector3) -> (f32, f32) {
        let phi = p.z.atan2(p.x);
        let theta = p.y.asin();
        let u = 1.0 - (phi + f32::consts::PI) / (2.0 * f32::consts::PI);
        let v = (theta + f32::consts::PI / 2.0) / f32::consts::PI;

        (u, v)
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = Vector3::dot(ray.direction(), ray.direction());
        let b = Vector3::dot(oc, ray.direction());
        let c = Vector3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = ray.point_at_parameter(temp);
                let (u, v) = Sphere::get_sphere_uv((p - self.center) / self.radius);
                let normal = (p - self.center) / self.radius;
                
                return Some(HitRecord::new(t, p, u, v, normal, &self.material));
            }

            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = ray.point_at_parameter(temp);
                let (u, v) = Sphere::get_sphere_uv((p - self.center) / self.radius);
                let normal = (p - self.center) / self.radius;

                return Some(HitRecord::new(t, p, u, v, normal, &self.material));
            }
        }

        None
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let min = self.center - Vector3::new(self.radius, self.radius, self.radius);
        let max = self.center + Vector3::new(self.radius, self.radius, self.radius);
        Some(AABB::new(min, max))
    }
}

pub struct MovingSphere {
    center0: Vector3,
    center1: Vector3,
    radius: f32,
    material: Box<Material>,
    time0: f32,
    time1: f32,
}

impl MovingSphere {
    pub fn new(center0: Vector3, center1: Vector3, time0: f32, time1: f32, radius: f32, material: Box<Material>) -> Self {
        MovingSphere {
            center0,
            center1,
            radius,
            material,
            time0,
            time1,
        }
    }

    fn center_at_time(&self, t: f32) -> Vector3 {
        self.center0 + ((t - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let center = self.center0 + ((ray.time() - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0);
        let oc = ray.origin() - center;
        let a = Vector3::dot(ray.direction(), ray.direction());
        let b = Vector3::dot(oc, ray.direction());
        let c = Vector3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = ray.point_at_parameter(temp);
                let (u, v) = Sphere::get_sphere_uv((p - center) / self.radius);
                let normal = (p - center) / self.radius;

                return Some(HitRecord::new(t, p, u, v, normal, &self.material));
            }

            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = ray.point_at_parameter(temp);
                let (u, v) = Sphere::get_sphere_uv((p - center) / self.radius);
                let normal = (p - center) / self.radius;

                return Some(HitRecord::new(t, p, u, v, normal, &self.material));
            }
        }

        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let center = self.center_at_time(t0);
        let min = center - Vector3::new(self.radius, self.radius, self.radius);
        let max = center + Vector3::new(self.radius, self.radius, self.radius);
        let b0 = AABB::new(min, max);

        let center = self.center_at_time(t1);
        let min = center - Vector3::new(self.radius, self.radius, self.radius);
        let max = center + Vector3::new(self.radius, self.radius, self.radius);
        let b1 = AABB::new(min, max);

        Some(AABB::surrounding_box(b0, b1))
    }
}