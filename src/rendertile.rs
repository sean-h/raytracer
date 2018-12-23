extern crate image;

use image::{ImageBuffer, Rgba};

pub struct RenderTile {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    pub image: Box<ImageBuffer<Rgba<u8>, Vec<u8>>>,
}

impl RenderTile {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        let image = ImageBuffer::new(width, height);

        RenderTile {
            x,
            y,
            width,
            height,
            image: Box::new(image),
        }
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}