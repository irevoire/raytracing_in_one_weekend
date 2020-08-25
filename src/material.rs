use crate::*;

pub trait Material {
    fn scatter(&self, r_in: Ray, rec: HitRecord, attenuation: Color) -> Option<Ray>;
}
