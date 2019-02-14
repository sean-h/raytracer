use tdmath::Ray;
use hitable::triangle::Triangle;
use material::{Material, NoMaterial};
use hitable::{Hitable, HitRecord};
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use modelloader::parse_obj_file;
use aabb::AABB;

pub struct Mesh {
    triangles: Vec<Triangle>,
    material: Box<Material>,
    bounding_box: Option<AABB>,
}

impl Mesh {
    pub fn new(filepath: &Path, material: Box<Material>) -> Self {
        let mut f = File::open(filepath).expect(&format!("File not found: {:?}", filepath));
        let mut file_contents = String::new();
        f.read_to_string(&mut file_contents).expect(&format!("Error reading file: {:?}", filepath));

        let mut triangles = Vec::new();
        let m = parse_obj_file(&file_contents);
        for v in m.vertices.chunks(3) {
            let triangle = Triangle::new(v[0].p, v[1].p, v[2].p, Box::new(NoMaterial::new()));
            triangles.push(triangle);
        }

        Mesh {
            triangles,
            material,
            bounding_box: None,
        }
    }
}

impl Hitable for Mesh {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for hitable in &self.triangles {
            match hitable.hit(ray, t_min, closest_so_far) {
                Some(mut hit) => {
                    closest_so_far = hit.t();
                    hit.material = &*self.material;
                    hit_record = Some(hit);
                },
                None => ()
            }
        }

        hit_record
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.bounding_box
    }
}