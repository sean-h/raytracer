pub mod scatterrecord;
pub mod lambertian;
pub mod dielectric;
pub mod metal;
pub mod diffuselight;
pub mod nomaterial;

pub use self::scatterrecord::{ScatterRecord, ScatterType};
pub use self::lambertian::Lambertian;
pub use self::dielectric::Dielectric;
pub use self::metal::Metal;
pub use self::diffuselight::DiffuseLight;
pub use self::nomaterial::NoMaterial;

use tdmath::{Vector3, Ray};
use hitable::HitRecord;

pub trait Material: Send + Sync {
    fn scatter(&self, _ray: Ray, _hit_record: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn scattering_pdf(&self, _ray: Ray, _hit_record: &HitRecord, _scattered: Ray) -> f32 {
        0.0
    }

    fn emit(&self, _ray: Ray, _hit: &HitRecord, _u: f32, _v: f32, _p: Vector3) -> Vector3 {
        Vector3::zero()
    }

    fn sample(&self) -> bool {
        false
    }
}