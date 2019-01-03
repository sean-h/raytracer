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
mod pdf;

use tdmath::{Vector3, Ray};
use hitable::{Hitable, HitableList};
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
use rect::XZRect;
use pdf::*;
use material::Lambertion;
use texture::ConstantTexture;
use sphere::Sphere;

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
                let c = color(r, &world, 0);

                if !c.has_nans() {
                    col = col + c;
                }
            }
            col = col / samples as f32;
            col = Vector3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());

            let ir = clampf(255.99 * col.r(), 0.0, 255.0) as u8;
            let ig = clampf(255.99 * col.g(), 0.0, 255.0) as u8;
            let ib = clampf(255.99 * col.b(), 0.0, 255.0) as u8;

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
                        if let Some(specular_ray) = scatter.specular_ray() {
                            return scatter.attenuation() * color(specular_ray, world, depth+1);
                        }

                        let attenuation = scatter.attenuation();

                        let fake_material = Arc::new(Lambertion::new(Box::new(ConstantTexture::new(Vector3::zero()))));
                        let light_shape: Arc<Hitable> = Arc::new(XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, fake_material.clone()));
                        let sphere_shape: Arc<Hitable> = Arc::new(Sphere::new(Vector3::new(190.0, 90.0, 190.0), 90.0, fake_material.clone()));
                        let importance_objects: Arc<Hitable> = Arc::new(HitableList::new(vec![light_shape, sphere_shape]));

                        let p_importance: Box<PDF> = Box::new(HitablePDF::new(hit.p(), importance_objects.clone()));
                        let p = match scatter.pdf() {
                            Some(pdf) => {
                                Box::new(MixturePDF::new(p_importance, pdf))
                            },
                            None => p_importance
                        };

                        let scattered = Ray::new(hit.p(), p.generate(), ray.time());
                        let pdf_val = p.value(scattered.direction());
                        let scattering_pdf = hit.material().scattering_pdf(ray, &hit, scattered);

                        return emitted + attenuation * scattering_pdf * color(scattered, world, depth+1) / pdf_val;
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

fn clampf(val: f32, min: f32, max: f32) -> f32 {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}