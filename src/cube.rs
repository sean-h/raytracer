use tdmath::{Vector3, Ray};
use hitable::*;
use rect::*;
use material::{Material, Lambertion};
use texture::ConstantTexture;
use aabb::AABB;

pub struct Cube {
    min: Vector3,
    max: Vector3,
    faces: Vec<Box<Hitable>>,
}

impl Cube {
    pub fn new(min: Vector3, max: Vector3, material: Box<Material>) -> Cube {
        let mut faces: Vec<Box<Hitable>> = Vec::new();

        let mat1 = Lambertion::new(Box::new(ConstantTexture::new(Vector3::new(0.73, 0.73, 0.73))));
        let mat2 = Lambertion::new(Box::new(ConstantTexture::new(Vector3::new(0.73, 0.73, 0.73))));
        let mat3 = Lambertion::new(Box::new(ConstantTexture::new(Vector3::new(0.73, 0.73, 0.73))));
        let mat4 = Lambertion::new(Box::new(ConstantTexture::new(Vector3::new(0.73, 0.73, 0.73))));
        let mat5 = Lambertion::new(Box::new(ConstantTexture::new(Vector3::new(0.73, 0.73, 0.73))));
        let mat6 = Lambertion::new(Box::new(ConstantTexture::new(Vector3::new(0.73, 0.73, 0.73))));

        faces.push(Box::new(XYRect::new(min.x, max.x, min.y, max.y, max.z, Box::new(mat1))));
        faces.push(Box::new(FlipNormals::new(Box::new(XYRect::new(min.x, max.x, min.y, max.y, min.z, Box::new(mat2))))));
        faces.push(Box::new(XZRect::new(min.x, max.x, min.z, max.z, max.y, Box::new(mat3))));
        faces.push(Box::new(FlipNormals::new(Box::new(XZRect::new(min.x, max.x, min.z, max.z, min.y, Box::new(mat4))))));
        faces.push(Box::new(YZRect::new(min.y, max.y, min.z, max.z, max.x, Box::new(mat5))));
        faces.push(Box::new(FlipNormals::new(Box::new(YZRect::new(min.y, max.y, min.z, max.z, min.x, Box::new(mat6))))));

        Cube {
            min,
            max,
            faces,
        }
    }
}

impl Hitable for Cube {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for hitable in &self.faces {
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
        Some(AABB::new(self.min, self.max))
    }
}