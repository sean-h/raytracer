use tdmath::{Vector3, Ray};
use pdf::PDF;

pub struct ScatterRecord {
    attenuation: Vector3,
    specular_ray: Option<Ray>,
    pdf: Option<Box<PDF>>,
}

impl ScatterRecord {
    pub fn new(attenuation: Vector3, specular_ray: Option<Ray>, pdf: Option<Box<PDF>>) -> Self {
        ScatterRecord {
            attenuation,
            specular_ray,
            pdf,
        }
    }

    pub fn attenuation(&self) -> Vector3 {
        self.attenuation
    }

    pub fn specular_ray(&self) -> Option<Ray> {
        self.specular_ray
    }

    pub fn is_specular(&self) -> bool {
        match self.specular_ray {
            Some(_) => true,
            None => false
        }
    }

    pub fn pdf(self) -> Option<Box<PDF>> {
        self.pdf
    }
}
