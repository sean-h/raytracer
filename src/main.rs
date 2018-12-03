extern crate rand;
extern crate tdmath;

mod hitable;
mod sphere;
mod world;
mod camera;
mod material;
mod aabb;
mod bvh;
mod texture;
mod noise;

use std::fs::File;
use std::io::prelude::*;
use tdmath::{Vector3, Ray};
use hitable::Hitable;
use world::World;
use camera::Camera;
use rand::Rng;
use std::time::{SystemTime};
use material::*;
use sphere::{Sphere, MovingSphere};
use texture::*;
use noise::Perlin;

fn main() -> std::io::Result<()> {
    let now = SystemTime::now();
    let nx = 200;
    let ny = 100;
    let ns = 100;

    let mut file = File::create("image.ppm")?;
    file.write(format!("P3\n{} {}\n255\n", nx, ny).as_bytes()).expect("Unable to write to file");;

    let world = random_scene();

    let lookfrom = Vector3::new(13.0, 2.0, 3.0);
    let lookat = Vector3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let camera = Camera::new(lookfrom, lookat, Vector3::up(), 20.0, nx as f32 / ny as f32, aperture, dist_to_focus, 0.0, 1.0);
    let mut rng = rand::thread_rng();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vector3::zero();
            for s in 0..ns {
                let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let v = (j as f32 + rng.gen::<f32>()) / ny as f32;

                let r = camera.get_ray(u, v);
                col = col + color(r, &world, 0);
            }
            col = col / ns as f32;
            col = Vector3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());

            let ir = (255.99 * col.r()) as i32;
            let ig = (255.99 * col.g()) as i32;
            let ib = (255.99 * col.b()) as i32;

            file.write(format!("{} {} {}\n", ir, ig, ib).as_bytes()).expect("Unable to write to file");
        }
    }

    match now.elapsed() {
        Ok(t) => println!("Took {} seconds to render", t.as_secs()),
        Err(e) => println!("Unable to determine render time: {}", e),
    }

    Ok(())
}

fn color(ray: Ray, world: &Box<Hitable>, depth: i32) -> Vector3 {
    let hit_record = world.hit(ray, 0.001, std::f32::MAX);

    match hit_record {
        Some(hit) => {
            if depth < 50 {
                match hit.material().scatter(ray, &hit) {
                    Some(scatter) => {
                        return scatter.attenuation() * color(scatter.scattered(), world, depth+1);
                    },
                    None => return Vector3::zero(),
                }
            } else {
                return Vector3::zero();
            }
        },
        None => {
            let unit_direction = ray.direction().normalized();
            let t = 0.5 * (unit_direction.y + 1.0);
            return Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0) * t;
        }
    }
}

fn random_scene() -> Box<Hitable> {
    let mut world = World::new();
    let mut rng = rand::thread_rng();

    let texture1 = ConstantTexture::new(Vector3::new(0.2, 0.3, 0.1));
    let texture2 = ConstantTexture::new(Vector3::new(0.9, 0.9, 0.9));
    //let checker = CheckerTexture::new(Box::new(texture1), Box::new(texture2));
    let perlin = Perlin::new();
    let noise_texture = NoiseTexture::new(Box::new(perlin));
    let ground_material = Lambertion::new(Box::new(noise_texture));
    let ground = Sphere::new(Vector3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(ground_material));
    world.add_hitable(Box::new(ground));

    /*for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f32>();
            let center = Vector3::new(a as f32 + 0.9 * rng.gen::<f32>(), 0.2, b as f32 + 0.9 * rng.gen::<f32>());
            if (center - Vector3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let color = Vector3::new(rng.gen::<f32>() * rng.gen::<f32>(), rng.gen::<f32>() * rng.gen::<f32>(), rng.gen::<f32>() * rng.gen::<f32>());
                    let center2 = center + Vector3::new(0.0, 0.5 * rng.gen::<f32>(), 0.0);
                    let albedo = ConstantTexture::new(color);
                    let sphere = MovingSphere::new(center, center2, 0.0, 1.0, 0.2, Box::new(Lambertion::new(Box::new(albedo))));
                    world.add_hitable(Box::new(sphere));
                } else if choose_mat < 0.95 {
                    let color = Vector3::new(0.5 * (1.0 + rng.gen::<f32>()), 0.5 * (1.0 + rng.gen::<f32>()), 0.5 * (1.0 + rng.gen::<f32>()));
                    let sphere = Sphere::new(center, 0.2, Box::new(Metal::new(color, 0.5 * rng.gen::<f32>())));
                    world.add_hitable(Box::new(sphere));
                } else {
                    let sphere = Sphere::new(center, 0.2, Box::new(Dielectric::new(1.5)));
                    world.add_hitable(Box::new(sphere));
                }
            }
        }
    }*/

    let sphere1 = Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, Box::new(Dielectric::new(1.5)));
    let sphere2 = Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0, Box::new(Lambertion::new(Box::new(ConstantTexture::new(Vector3::new(0.4, 0.2, 0.1))))));
    let sphere3 = Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0, Box::new(Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0)));
    world.add_hitable(Box::new(sphere1));
    world.add_hitable(Box::new(sphere2));
    world.add_hitable(Box::new(sphere3));

    Box::new(world)
}