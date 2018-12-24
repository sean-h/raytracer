pub mod scatterrecord;
pub mod lambertian;
pub mod dielectric;
pub mod metal;
pub mod diffuselight;

pub use self::scatterrecord::ScatterRecord;
pub use self::lambertian::Lambertion;
pub use self::dielectric::Dielectric;
pub use self::metal::Metal;
pub use self::diffuselight::DiffuseLight;

use tdmath::{Vector3, Ray};
use hitable::HitRecord;

pub trait Material: Send + Sync {
    fn scatter(&self, ray: Ray, hit_record: &HitRecord) -> Option<ScatterRecord>;

    fn emit(&self, _u: f32, _v: f32, _p: Vector3) -> Vector3 {
        Vector3::zero()
    }
}