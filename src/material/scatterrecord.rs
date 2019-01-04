use tdmath::{Vector3, Ray};
use pdf::PDF;

pub enum ScatterType {
    Specular(Ray),
    Scatter(Box<PDF>)
}

pub struct ScatterRecord {
    attenuation: Vector3,
    scatter_type: ScatterType
}

impl ScatterRecord {
    pub fn new(attenuation: Vector3, scatter_type: ScatterType) -> Self {
        ScatterRecord {
            attenuation,
            scatter_type,
        }
    }

    pub fn attenuation(&self) -> Vector3 {
        self.attenuation
    }

    pub fn scatter_type(self) -> ScatterType {
        self.scatter_type
    }
}
