use tdmath::Ray;
use hitable::triangle::Triangle;
use material::{Material, NoMaterial};
use hitable::{Hitable, HitRecord};
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use modelloader::parse_obj_file;
use aabb::AABB;
use bvh::BVH;

pub struct Mesh {
    material: Box<Material>,
    bounding_box: Option<AABB>,
    bvh: BVH,
}

impl Mesh {
    pub fn new(filepath: &Path, material: Box<Material>) -> Self {
        let mut f = File::open(filepath).expect(&format!("File not found: {:?}", filepath));
        let mut file_contents = String::new();
        f.read_to_string(&mut file_contents).expect(&format!("Error reading file: {:?}", filepath));

        let mut triangles: Vec<Box<Hitable>> = Vec::new();
        let m = parse_obj_file(&file_contents);
        for v in m.vertices.chunks(3) {
            let triangle = Triangle::new(v[0].p, v[1].p, v[2].p, Box::new(NoMaterial::new()));
            triangles.push(Box::new(triangle));
        }

        let bvh = BVH::from_list(triangles, 0.0, 0.0);

        Mesh {
            material,
            bounding_box: None,
            bvh,
        }
    }
}

impl Hitable for Mesh {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self.bvh.hit(ray, t_min, t_max) {
            Some(mut hit) => {
                hit.material = &*self.material;
                Some(hit)
            },
            None => None
        }
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        self.bounding_box
    }
}