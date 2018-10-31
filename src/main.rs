extern crate rand;

mod vector3;
mod ray;
mod hitable;
mod sphere;
mod world;
mod camera;

use std::fs::File;
use std::io::prelude::*;
use vector3::Vector3;
use ray::Ray;
use hitable::Hitable;
use sphere::Sphere;
use world::World;
use camera::Camera;
use rand::Rng;

fn main() -> std::io::Result<()> {
    let nx = 200;
    let ny = 100;
    let ns = 100;

    let mut file = File::create("image.ppm")?;
    file.write(format!("P3\n{} {}\n255\n", nx, ny).as_bytes()).expect("Unable to write to file");

    let lower_left_corner = Vector3::new(-2.0, -1.0, -1.0);
    let horizontal = Vector3::new(4.0, 0.0, 0.0);
    let vertical = Vector3::new(0.0, 2.0, 0.0);
    let origin = Vector3::zero();

    let mut world = World::new();
    let sphere1 = Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5);
    let sphere2 = Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0);
    world.add_hitable(Box::new(sphere1));
    world.add_hitable(Box::new(sphere2));

    let camera = Camera::new();
    let mut rng = rand::thread_rng();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vector3::zero();
            for s in 0..ns {
                let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let v = (j as f32 + rng.gen::<f32>()) / ny as f32;

                let r = camera.get_ray(u, v);
                col = col + color(r, &world);
            }
            col = col / ns as f32;

            let ir = (255.99 * col.r()) as i32;
            let ig = (255.99 * col.g()) as i32;
            let ib = (255.99 * col.b()) as i32;

            file.write(format!("{} {} {}\n", ir, ig, ib).as_bytes()).expect("Unable to write to file");
        }
    }

    Ok(())
}

fn color(ray: Ray, world: &Hitable) -> Vector3 {
    let hit_record = world.hit(ray, 0.0, std::f32::MAX);

    match hit_record {
        Some(hit) => {
            return Vector3::new(hit.normal().x + 1.0, hit.normal().y + 1.0, hit.normal().z + 1.0) * 0.5;
        },
        None => {
            let unit_direction = ray.direction().normalized();
            let t = 0.5 * (unit_direction.y + 1.0);
            return Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0) * t;
        }
    }
}

fn hit_sphere(center: Vector3, radius: f32, r: Ray) -> f32 {
    let oc = r.origin() - center;
    let a = Vector3::dot(r.direction(), r.direction());
    let b = 2.0 * Vector3::dot(oc, r.direction());
    let c = Vector3::dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}