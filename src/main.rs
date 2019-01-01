extern crate rand;
extern crate tdmath;
extern crate cmdpro;
extern crate image;
extern crate toml;
extern crate threadpool;

mod hitable;
mod sphere;
mod world;
mod camera;
mod material;
mod aabb;
mod bvh;
mod texture;
mod noise;
mod settings;
mod rect;
mod cube;
mod transform;
mod volume;
mod rendertile;
mod onb;

use tdmath::{Vector3, Ray};
use hitable::Hitable;
use world::World;
use camera::Camera;
use rand::Rng;
use std::time::{SystemTime};
use cmdpro::{CommandLineProcessor, ParameterType};
use settings::Settings;
use image::{Rgba, GenericImage, DynamicImage};
use std::fs::{File};
use std::io::prelude::*;
use toml::Value;
use rendertile::RenderTile;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::Arc;

fn main() {
    let mut command_line_processor = CommandLineProcessor::new();
    command_line_processor.add_parameter("width", ParameterType::UInteger, vec!["--width".to_owned(), "--w".to_owned()]);
    command_line_processor.add_parameter("height", ParameterType::UInteger, vec!["--height".to_owned(), "--h".to_owned()]);
    command_line_processor.add_parameter("samples", ParameterType::UInteger, vec!["--samples".to_owned(), "--s".to_owned()]);
    command_line_processor.add_parameter("output", ParameterType::Path, vec!["--output".to_owned(), "--o".to_owned()]);
    command_line_processor.add_parameter("scene", ParameterType::Path, vec!["--scene".to_owned(), "--S".to_owned()]);
    command_line_processor.add_parameter("threads", ParameterType::UInteger, vec!["--threads".to_owned(), "--t".to_owned()]);
    command_line_processor.parse_command_line();

    if command_line_processor.abort_flag() {
        return;
    }

    let settings = Settings::from_commandline(&command_line_processor);
    let mut scene_buffer = String::new();
    let mut scene = File::open(settings.scene_path()).expect("Unable to load scene");
    scene.read_to_string(&mut scene_buffer).expect("Unable to read scene file");
    let scene = scene_buffer.parse::<Value>().expect("Unable to parse scene file");

    let now = SystemTime::now();
    let nx = settings.width();
    let ny = settings.height();
    let ns = settings.samples();

    let world: Box<Hitable> = Box::new(World::from_toml(&scene));
    let world = Arc::new(world);
    let camera = Camera::from_toml(&scene["camera"], nx as f32 / ny as f32);

    let mut tiles = Vec::new();
    tiles.push(RenderTile::new(0, 0, nx / 2, ny / 2));
    tiles.push(RenderTile::new(nx / 2, 0, nx / 2, ny / 2));
    tiles.push(RenderTile::new(0, ny / 2, nx / 2, ny / 2));
    tiles.push(RenderTile::new(nx / 2, ny / 2, nx / 2, ny / 2));

    let pool = ThreadPool::new(settings.threads() as usize);
    let (tx, rx): (Sender<RenderTile>, Receiver<RenderTile>) = channel();

    // Render tiles
    for mut tile in tiles {
        let tx = tx.clone();
        let world = Arc::clone(&world);

        pool.execute(move || {
            render_tile(&mut tile, &camera, &world, nx, ny, ns);
            tx.send(tile).expect("Unable to send data");
        });
    }
    drop(tx);

    // Draw tiles to result
    let mut image = DynamicImage::new_rgb8(nx, ny);
    for tile in rx.iter() {
        image.copy_from(&*tile.image, tile.x(), tile.y());
    }

    match now.elapsed() {
        Ok(t) => println!("Took {} seconds to render", t.as_secs()),
        Err(e) => println!("Unable to determine render time: {}", e),
    }

    let image = image.flipv();
    image.save(settings.export_path()).unwrap();
}

fn render_tile(tile: &mut RenderTile, camera: &Camera, world: &Box<Hitable>, image_width: u32, image_height: u32, samples: u32) {
    let x = tile.x();
    let y = tile.y();
    let x_end = x + tile.width();
    let y_end = y + tile.height();
    let mut rng = rand::thread_rng();

    for j in y..y_end {
        for i in x..x_end {
            let mut col = Vector3::zero();
            for _ in 0..samples {
                let u = (i as f32 + rng.gen::<f32>()) / image_width as f32;
                let v = (j as f32 + rng.gen::<f32>()) / image_height as f32;

                let r = camera.get_ray(u, v);
                col = col + color(r, &world, 0);
            }
            col = col / samples as f32;
            col = Vector3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());

            let ir = (255.99 * col.r()) as u8;
            let ig = (255.99 * col.g()) as u8;
            let ib = (255.99 * col.b()) as u8;

            tile.image.put_pixel(i - x, j - y, Rgba { data: [ir, ig, ib, 255] })
        }
    }
}

fn color(ray: Ray, world: &Box<Hitable>, depth: i32) -> Vector3 {
    let hit_record = world.hit(ray, 0.001, std::f32::MAX);
    

    match hit_record {
        Some(hit) => {
            let emitted = hit.material().emit(ray, &hit, hit.u(), hit.v(), hit.p());
            if depth < 50 {
                match hit.material().scatter(ray, &hit) {
                    Some(scatter) => {
                        // TEMP - Send all rays to the light
                        let mut rng = rand::thread_rng();
                        let l0 = Vector3::new(213.0, 554.0, 227.0);
                        let l1 = Vector3::new(343.0, 554.0, 332.0);
                        let x = l0.x + rng.gen::<f32>() * (l1.x - l0.x);
                        let y = l0.y;
                        let z = l0.z + rng.gen::<f32>() * (l1.z - l0.z);                        
                        let random_point_on_light = Vector3::new(x, y, z);
                        let to_light = random_point_on_light - hit.p();
                        let distance_squared = to_light.length_squared();
                        let to_light = to_light.normalized();
                        if Vector3::dot(to_light, hit.normal()) < 0.0 {
                            return emitted;
                        }
                        
                        let light_cosine = to_light.y.abs();
                        if light_cosine < 0.000001 {
                            return emitted;
                        }

                        
                        let light_area = (l1.x - l0.x) * (l1.z - l0.z);
                        let pdf = distance_squared / (light_cosine * light_area);
                        let scattered = Ray::new(hit.p(), to_light, ray.time());
                        let mat_pdf = hit.material().scattering_pdf(ray, &hit, scattered);
                        return emitted + mat_pdf * scatter.attenuation() * color(scattered, world, depth+1) / pdf;
                        // END TEMP

                        //let pdf = hit.material().scattering_pdf(ray, &hit, scatter.scattered());
                        //return emitted + pdf * scatter.attenuation() * color(scatter.scattered(), world, depth+1) / pdf;
                    },
                    None => return emitted,
                }
            } else {
                return emitted;
            }
        },
        None => {
            return Vector3::zero();
        }
    }
}
