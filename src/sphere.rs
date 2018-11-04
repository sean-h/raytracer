use hitable::*;
use vector3::Vector3;
use ray::Ray;
use material::Material;

pub struct Sphere {
    center: Vector3,
    radius: f32,
    material: Box<Material>,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f32, material: Box<Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
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

                return Some(HitRecord::new(t, p, normal, &self.material));
            }

            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = ray.point_at_parameter(temp);
                let normal = (p - self.center) / self.radius;

                return Some(HitRecord::new(t, p, normal, &self.material));
            }
        }

        None
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
    pub fn new(center0: Vector3, center1: Vector3, time0: f32, time1: f32, radius: f32, material: Box<Material>) -> MovingSphere {
        MovingSphere {
            center0,
            center1,
            radius,
            material,
            time0,
            time1,
        }
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
                let normal = (p - center) / self.radius;

                return Some(HitRecord::new(t, p, normal, &self.material));
            }

            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = ray.point_at_parameter(temp);
                let normal = (p - center) / self.radius;

                return Some(HitRecord::new(t, p, normal, &self.material));
            }
        }

        None
    }
}