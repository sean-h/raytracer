use tdmath::{Vector3, Ray};

pub struct ScatterRecord {
    attenuation: Vector3,
    scattered: Ray,
}

impl ScatterRecord {
    pub fn new(attenuation: Vector3, scattered: Ray) -> Self {
        ScatterRecord {
            attenuation,
            scattered,
        }
    }

    pub fn attenuation(&self) -> Vector3 {
        self.attenuation
    }

    pub fn scattered(&self) -> Ray {
        self.scattered
    }
}

