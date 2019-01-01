pub mod cosinepdf;
pub mod hitablepdf;
pub mod mixturepdf;

use tdmath::Vector3;
pub use self::cosinepdf::CosinePDF;
pub use self::hitablepdf::HitablePDF;
pub use self::mixturepdf::MixturePDF;

pub trait PDF {
    fn value(&self, direction: Vector3) -> f32;
    fn generate(&self) -> Vector3;
}