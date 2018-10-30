use hitable::*;
use vector3::Vector3;
use ray::Ray;

pub struct Sphere {
    center: Vector3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f32) -> Sphere {
        Sphere {
            center,
            radius,
        }
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
                let normal = (p - self.center) / self.radius;

                return Some(HitRecord::new(t, p, normal));
            }

            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = ray.point_at_parameter(temp);
                let normal = (p - self.center) / self.radius;

                return Some(HitRecord::new(t, p, normal));
            }
        }

        None
    }
}