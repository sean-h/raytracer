extern crate tdmath;
extern crate toml;

use hitable::*;
use tdmath::{Ray, Vector3};
use aabb::AABB;
use self::toml::Value;
use sphere::Sphere;
use material::*;
use texture::*;
use noise::Perlin;
use rect::{XYRect, XZRect, YZRect};
use cube::Cube;
use transform::{Translate, RotateY};
use std::sync::Arc;

pub struct World {
    hitables: Vec<Box<Hitable>>,
}

impl World {
    pub fn from_toml(scene: &Value) -> Self {
        let mut hitables: Vec<Box<Hitable>> = Vec::new();

        let objects = scene["objects"].as_table().unwrap();
        for (_, obj_data) in objects.iter() {
            let obj_type = obj_data["type"].as_str().unwrap();
            let material_name = obj_data["material"].as_str().unwrap();
            let material_data = &scene["materials"].as_table().unwrap()[material_name];
            let material = World::create_material_from_toml(material_data, &scene["textures"]);

            let hitable = World::create_object_from_toml(obj_type, obj_data, material);
            hitables.push(hitable);
        }

        World {
            hitables,
        }
    }

    fn create_material_from_toml(material_data: &Value, textures: &Value) -> Arc<Material> {
        let material_type = material_data["type"].as_str().unwrap();
        
        if material_type == "lambertian" {
            let texture_name = material_data["texture"].as_str().unwrap();
            let texture_data = &textures[texture_name];
            let texture = World::create_texture_from_toml(texture_data);
            Arc::new(Lambertion::new(texture))
        } else if material_type == "dielectric" {
            let ref_index = material_data["ref_index"].as_float().unwrap() as f32;
            Arc::new(Dielectric::new(ref_index))
        } else if material_type == "metal" {
            let albedo = material_data["albedo"].as_array().unwrap();
            let r = albedo[0].as_float().unwrap() as f32;
            let g = albedo[1].as_float().unwrap() as f32;
            let b = albedo[2].as_float().unwrap() as f32;
            let fuzz = material_data["fuzz"].as_float().unwrap() as f32;
            Arc::new(Metal::new(Vector3::new(r, g, b), fuzz))
        } else if material_type == "diffuse_light" {
            let texture_name = material_data["texture"].as_str().unwrap();
            let texture_data = &textures[texture_name];
            let texture = World::create_texture_from_toml(texture_data);
            Arc::new(DiffuseLight::new(texture))
        } else {
            panic!("Unknown material type")
        }
    }

    fn create_texture_from_toml(texture_data: &Value) -> Box<Texture> {
        let texture_type = texture_data["type"].as_str().unwrap();

        if texture_type == "constant" {
            let color = texture_data["color"].as_array().unwrap();
            let r = color[0].as_float().unwrap() as f32;
            let g = color[1].as_float().unwrap() as f32;
            let b = color[2].as_float().unwrap() as f32;
            Box::new(ConstantTexture::new(Vector3::new(r, g, b)))
        } else if texture_type == "perlin" {
            let perlin = Perlin::new();
            let scale = texture_data["scale"].as_float().unwrap() as f32;
            let turbulence = texture_data["turbulence"].as_integer().unwrap() as u32;
            Box::new(NoiseTexture::new(Box::new(perlin), scale, turbulence))
        } else {
            panic!("Unknown texture type")
        }
    }

    fn create_object_from_toml(obj_type: &str, obj_data: &Value, material: Arc<Material>) -> Box<Hitable> {
        if obj_type == "sphere" {
            let position = obj_data["position"].as_array().unwrap();
            let x = position[0].as_float().unwrap() as f32;
            let y = position[1].as_float().unwrap() as f32;
            let z = position[2].as_float().unwrap() as f32;
            let radius = obj_data["radius"].as_float().unwrap() as f32;

            let sphere: Box<Hitable> = Box::new(Sphere::new(Vector3::new(x, y, z), radius, material));

            sphere
        } else if obj_type == "xyrect" {
            let bounds = obj_data["bounds"].as_array().unwrap();
            let x0 = bounds[0].as_float().unwrap() as f32;
            let x1 = bounds[1].as_float().unwrap() as f32;
            let y0 = bounds[2].as_float().unwrap() as f32;
            let y1 = bounds[3].as_float().unwrap() as f32;
            let k = obj_data["k"].as_float().unwrap() as f32;

            let rect = XYRect::new(x0, x1, y0, y1, k, material);
            let flip = obj_data["flip"].as_bool().unwrap_or(false);
            if flip {
                let flipped = FlipNormals::new(Box::new(rect));
                Box::new(flipped)
            } else {
                let rect: Box<Hitable> = Box::new(rect);
                rect
            }
        } else if obj_type == "xzrect" {
            let bounds = obj_data["bounds"].as_array().unwrap();
            let x0 = bounds[0].as_float().unwrap() as f32;
            let x1 = bounds[1].as_float().unwrap() as f32;
            let z0 = bounds[2].as_float().unwrap() as f32;
            let z1 = bounds[3].as_float().unwrap() as f32;
            let k = obj_data["k"].as_float().unwrap() as f32;

            let rect = XZRect::new(x0, x1, z0, z1, k, material);
            
            let flip = obj_data["flip"].as_bool().unwrap_or(false);
            if flip {
                let flipped = FlipNormals::new(Box::new(rect));
                Box::new(flipped)
            } else {
                let rect: Box<Hitable> = Box::new(rect);
                rect
            }
        } else if obj_type == "yzrect" {
            let bounds = obj_data["bounds"].as_array().unwrap();
            let y0 = bounds[0].as_float().unwrap() as f32;
            let y1 = bounds[1].as_float().unwrap() as f32;
            let z0 = bounds[2].as_float().unwrap() as f32;
            let z1 = bounds[3].as_float().unwrap() as f32;
            let k = obj_data["k"].as_float().unwrap() as f32;

            let rect = YZRect::new(y0, y1, z0, z1, k, material);

            let flip = obj_data["flip"].as_bool().unwrap_or(false);
            if flip {
                let flipped = FlipNormals::new(Box::new(rect));
                Box::new(flipped)
            } else {
                let rect: Box<Hitable> = Box::new(rect);
                rect
            }       
        } else if obj_type == "cube" {
            let min = obj_data["min"].as_array().unwrap();
            let x = min[0].as_float().unwrap() as f32;
            let y = min[1].as_float().unwrap() as f32;
            let z = min[2].as_float().unwrap() as f32;
            let min = Vector3::new(x, y, z);

            let max = obj_data["max"].as_array().unwrap();
            let x = max[0].as_float().unwrap() as f32;
            let y = max[1].as_float().unwrap() as f32;
            let z = max[2].as_float().unwrap() as f32;
            let max = Vector3::new(x, y, z);

            let cube = Box::new(Cube::new(min, max, material));

            let cube: Box<Hitable> = match obj_data.get("rotate_y") {
                Some(rotate_y) => {
                    let y = rotate_y.as_float().unwrap() as f32;
                    Box::new(RotateY::new(cube, y))
                },
                None => cube
            };

            let cube: Box<Hitable> = match obj_data.get("translate") {
                Some(translate) => {
                    let translate = translate.as_array().unwrap();
                    let x = translate[0].as_float().unwrap() as f32;
                    let y = translate[1].as_float().unwrap() as f32;
                    let z = translate[2].as_float().unwrap() as f32;

                    Box::new(Translate::new(cube, Vector3::new(x, y, z)))
                },
                None => cube
            };
            
            cube
        } else {
            panic!("Unknown object type");
        }
    }

    pub fn hitables(self) -> Vec<Box<Hitable>> {
        self.hitables
    }
}

impl Hitable for World {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for hitable in &self.hitables {
            match hitable.hit(ray, t_min, closest_so_far) {
                Some(hit) => {
                    closest_so_far = hit.t();
                    hit_record = Some(hit);
                },
                None => ()
            }
        }

        hit_record
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if self.hitables.len() == 0 {
            return None;
        }

        let mut bbox = match self.hitables[0].bounding_box(t0, t1) {
            Some(aabb) => aabb,
            None => return None,
        };

        for hitable in &self.hitables {
            let aabb = match hitable.bounding_box(t0, t1) {
                Some(aabb) => aabb,
                None => return None,
            };

            bbox = AABB::surrounding_box(bbox, aabb);
        }

        Some(bbox)
    }
}