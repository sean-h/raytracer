use vector3::Vector3;
use ray::Ray;

pub struct Camera {
    origin: Vector3,
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            origin: Vector3::zero(),
            lower_left_corner: Vector3::new(-2.0, -1.0, -1.0),
            horizontal: Vector3::new(4.0, 0.0, 0.0),
            vertical: Vector3::new(0.0, 2.0, 0.0),
        }
    }

    pub fn get_ray(&self,   u: f32, v: f32) -> Ray {
        Ray::new(self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin)
    }
}