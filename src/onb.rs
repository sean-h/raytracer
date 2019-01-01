use tdmath::Vector3;

// Ortho-normal Base
pub struct ONB {
    axis: [Vector3; 3],
}

impl ONB {
    pub fn from_w(n: Vector3) -> ONB {
        let w = n.normalized();
        let a = if w.x.abs() > 0.9 {
            Vector3::new(0.0, 1.0, 0.0)
        } else {
            Vector3::new(1.0, 0.0, 0.0)
        };

        let v = Vector3::cross(w, a).normalized();
        let u = Vector3::cross(w, v);

        ONB {
            axis: [u, v, w],
        }
    }

    pub fn u(&self) -> Vector3 {
        self.axis[0]
    }

    pub fn v(&self) -> Vector3 {
        self.axis[1]
    }

    pub fn w(&self) -> Vector3 {
        self.axis[2]
    }

    pub fn local(&self, v: Vector3) -> Vector3 {
        v.x * self.u() + v.y * self.v() + v.z * self.w()
    }
}