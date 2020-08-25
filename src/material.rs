use crate::*;

pub trait Material {
    fn scatter(&self, r_in: Ray, rec: HitRecord, attenuation: Color) -> Option<Ray>;
}

impl std::fmt::Debug for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Material")
    }
}
