extern crate rand;
extern crate tdmath;
extern crate toml;

use tdmath::Vector3;
use tdmath::Ray;
use std::f32::consts;
use rand::Rng;
use toml::Value;

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

    pub fn from_toml(toml: &Value, aspect: f32) -> Camera {
        let position = toml["position"].as_array().unwrap();
        let x = position[0].as_float().unwrap() as f32;
        let y = position[1].as_float().unwrap() as f32;
        let z = position[2].as_float().unwrap() as f32;
        let look_from = Vector3::new(x, y, z);

        let look = toml["look"].as_array().unwrap();
        let x = look[0].as_float().unwrap() as f32;
        let y = look[1].as_float().unwrap() as f32;
        let z = look[2].as_float().unwrap() as f32;
        let lookat = Vector3::new(x, y, z);

        let focus_dist = toml["focus_dist"].as_float().unwrap() as f32;
        let aperture = toml["aperture"].as_float().unwrap() as f32;
        let fov = toml["fov"].as_float().unwrap() as f32;
        let t0 = toml["t0"].as_float().unwrap() as f32;
        let t1 = toml["t1"].as_float().unwrap() as f32;

        Camera::new(look_from, lookat, Vector3::up(), fov, aspect, aperture, focus_dist, t0, t1)
    }
}