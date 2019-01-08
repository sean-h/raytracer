extern crate rand;
extern crate tdmath;
extern crate cmdpro;
extern crate image;
extern crate toml;
extern crate threadpool;

mod hitable;
mod world;
mod camera;
mod material;
mod aabb;
mod bvh;
mod texture;
mod noise;
mod settings;
mod transform;
mod rendertile;
mod onb;
mod pdf;

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
use pdf::*;
use material::ScatterType;

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

    let world = Arc::new(World::from_toml(&scene));
    let sample_world = Arc::new(World::from_toml_samples(&scene));
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
        let sample_world = Arc::clone(&sample_world);

        pool.execute(move || {
            render_tile(&mut tile, &camera, world, sample_world, nx, ny, ns);
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

fn render_tile(tile: &mut RenderTile, camera: &Camera, world: Arc<World>, sample_world: Arc<World>, image_width: u32, image_height: u32, samples: u32) {
    let x = tile.x();
    let y = tile.y();
    let x_end = x + tile.width();
    let y_end = y + tile.height();
    let mut rng = rand::thread_rng();

    let w: &Hitable = &*world;
    let s: &Hitable = &*sample_world;

    for j in y..y_end {
        for i in x..x_end {
            let mut col = Vector3::zero();
            for _ in 0..samples {
                let u = (i as f32 + rng.gen::<f32>()) / image_width as f32;
                let v = (j as f32 + rng.gen::<f32>()) / image_height as f32;

                let r = camera.get_ray(u, v);
                let c = color(r, w, s, 0);

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

fn color<'a>(ray: Ray, world: &'a Hitable, sample_world: &'a Hitable, depth: i32) -> Vector3 {
    match world.hit(ray, 0.001, std::f32::MAX) {
        Some(hit) => {
            let emitted = hit.material.emit(ray, &hit, hit.u(), hit.v(), hit.p());
            if depth < 50 {
                match hit.material.scatter(ray, &hit) {
                    Some(scatter) => {
                        let attenuation = scatter.attenuation();

                        match scatter.scatter_type() {
                            ScatterType::Specular(specular_ray) => return attenuation * color(specular_ray, world, sample_world, depth+1),
                            ScatterType::Scatter(pdf) => {
                                let (scattered, pdf_val, scattering_pdf) = {
                                    let p_importance = HitablePDF::new(hit.p(), sample_world);
                                    let p = MixturePDF::new(&p_importance, &*pdf);

                                    let scattered = Ray::new(hit.p(), p.generate(), ray.time());
                                    let pdf_val = p.value(scattered.direction());

                                    let scattering_pdf = hit.material.scattering_pdf(ray, &hit, scattered);

                                    (scattered, pdf_val, scattering_pdf)
                                };

                                return emitted + attenuation * scattering_pdf * color(scattered, world, sample_world, depth+1) / pdf_val;
                            }
                        }
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