use vector3::Vector3;
use ray::Ray;
use std::f32::consts;

pub struct Camera {
    origin: Vector3,
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
}

impl Camera {
    pub fn new(lookfrom: Vector3, lookat: Vector3, vup: Vector3, vfov: f32, aspect: f32) -> Camera {
        let theta = vfov * consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (lookfrom - lookat).normalized();
        let u = Vector3::cross(vup, w);
        let v = Vector3::cross(w, u);

        let lower_left = lookfrom - u * half_width - v * half_height - w;

        Camera {
            origin: lookfrom,
            lower_left_corner: lower_left,
            horizontal: u * half_width * 2.0,
            vertical: v * half_height * 2.0,
        }
    }

    pub fn get_ray(&self,   u: f32, v: f32) -> Ray {
        Ray::new(self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin)
    }
}