extern crate tdmath;

use tdmath::Vector3;

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: Vector3) -> Vector3;
}

pub struct ConstantTexture {
    color: Vector3,
}

impl ConstantTexture {
    pub fn new(color: Vector3) -> ConstantTexture {
        ConstantTexture {
            color
        }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, u: f32, v: f32, p: Vector3) -> Vector3 {
        self.color
    }
}

pub struct CheckerTexture {
    odd: Box<Texture>,
    even: Box<Texture>,
}

impl CheckerTexture {
    pub fn new(odd: Box<Texture>, even: Box<Texture>) -> CheckerTexture {
        CheckerTexture {
            odd,
            even,
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: Vector3) -> Vector3 {
        let sines = (p.x * 10.0).sin() * (p.y * 10.0).sin() * (p.z * 10.0).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}