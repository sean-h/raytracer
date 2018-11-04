use vector3::Vector3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin: Vector3,
    direction: Vector3,
    time: f32,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3, time: f32) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn origin(&self) -> Vector3 {
        self.origin
    }

    pub fn direction(self) -> Vector3 {
        self.direction
    }
    
    pub fn time(self) -> f32 {
        self.time
    }

    pub fn point_at_parameter(&self, t: f32) -> Vector3 {
        self.origin + self.direction * t
    }
}