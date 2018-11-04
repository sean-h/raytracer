extern crate rand;

use vector3::Vector3;
use ray::Ray;
use std::f32::consts;
use rand::Rng;

pub struct Camera {
    origin: Vector3,
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    lens_radius: f32,
    u: Vector3,
    v: Vector3,
    w: Vector3,
    time0: f32,
    time1: f32,
}

impl Camera {
    pub fn new(lookfrom: Vector3, lookat: Vector3, vup: Vector3, vfov: f32, aspect: f32, aperture: f32, focus_dist: f32, time0: f32, time1: f32) -> Camera {
        let theta = vfov * consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (lookfrom - lookat).normalized();
        let u = Vector3::cross(vup, w).normalized();
        let v = Vector3::cross(w, u);

        let lower_left = lookfrom - u * half_width * focus_dist - v * half_height * focus_dist - w * focus_dist;

        Camera {
            origin: lookfrom,
            lower_left_corner: lower_left,
            horizontal: u * half_width * 2.0 * focus_dist,
            vertical: v * half_height * 2.0 * focus_dist,
            lens_radius: aperture / 2.0,
            u: u,
            v: v,
            w: w,
            time0,
            time1,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = Vector3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;

        let mut rng = rand::thread_rng();
        let time = self.time0 + (self.time1 - self.time0) * rng.gen::<f32>();

        Ray::new(self.origin + offset,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset, time)
    }
}