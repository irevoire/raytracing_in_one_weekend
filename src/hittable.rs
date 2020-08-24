use crate::{Point3, Ray, Vec3};

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64) -> Self {
        Self { p, normal, t }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
}
