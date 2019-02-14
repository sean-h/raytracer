use material::Material;

pub struct NoMaterial {

}

impl NoMaterial {
    pub fn new() -> Self {
        NoMaterial {}
    }
}

impl Material for NoMaterial {

}