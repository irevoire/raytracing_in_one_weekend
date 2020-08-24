use crate::{Point3, Ray, Vec3};

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64, front_face: bool) -> Self {
        Self {
            p,
            normal,
            t,
            front_face,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.dir.dot(outward_normal) < 0.;
        self.normal = match self.front_face {
            true => outward_normal,
            false => -outward_normal,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
}
