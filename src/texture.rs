extern crate tdmath;
extern crate image;

use tdmath::Vector3;
use noise::Perlin;
use image::{ImageBuffer, Rgb};

pub trait Texture: Send + Sync {
    fn value(&self, u: f32, v: f32, p: Vector3) -> Vector3;
}

pub struct ConstantTexture {
    color: Vector3,
}

impl ConstantTexture {
    pub fn new(color: Vector3) -> Self {
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
    pub fn new(odd: Box<Texture>, even: Box<Texture>) -> Self {
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

pub struct NoiseTexture {
    perlin: Box<Perlin>,
    scale: f32,
    turb: u32,
}

impl NoiseTexture {
    pub fn new(perlin: Box<Perlin>, scale: f32, turb: u32) -> Self {
        NoiseTexture {
            perlin,
            scale,
            turb,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, p: Vector3) -> Vector3 {
        if self.turb > 0 {
            let noise = 1.0 + (self.scale * p.z + 10.0 * self.perlin.turb(p, 7)).sin();
            Vector3::new(1.0, 1.0, 1.0) * 0.5 * noise
        } else {
            Vector3::new(1.0, 1.0, 1.0) * self.perlin.turb(p * self.scale, 7)
        }
    }
}

pub struct ImageTexture {
    image: Box<ImageBuffer<Rgb<u8>, Vec<u8>>>
}

impl ImageTexture {
    pub fn new(image: Box<ImageBuffer<Rgb<u8>, Vec<u8>>>) -> Self {
        ImageTexture {
            image
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _p: Vector3) -> Vector3 {
        let (width, height) = self.image.dimensions();
        
        let i = u * width as f32;
        let i = if i < 0.0 {
            0.0
        } else if i > width as f32 - 1.0 {
            width as f32 - 1.0
        } else {
            i
        };


        let j = (1.0 - v) * height as f32 - 0.001;
        let j = if j < 0.0 {
            0.0
        } else if j > height as f32 - 1.0 {
            height as f32 - 1.0
        } else {
            j
        };

        let pixel = self.image.get_pixel(i as u32, j as u32);
        let r = pixel[0] as f32 / 255.0;
        let g = pixel[1] as f32 / 255.0;
        let b = pixel[2] as f32 / 255.0;

        Vector3::new(r, g, b)
    }
}