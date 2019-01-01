use tdmath::{Vector3, Ray};

pub struct ScatterRecord {
    attenuation: Vector3,
    scattered: Ray,
    pdf: f32,
}

impl ScatterRecord {
    pub fn new(attenuation: Vector3, scattered: Ray, pdf: f32) -> Self {
        ScatterRecord {
            attenuation,
            scattered,
            pdf,
        }
    }

    pub fn attenuation(&self) -> Vector3 {
        self.attenuation
    }

    pub fn scattered(&self) -> Ray {
        self.scattered
    }

    pub fn pdf(&self) -> f32 {
        self.pdf
    }
}
