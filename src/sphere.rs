use crate::{HitRecord, Hittable, Point3, Ray};

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let oc = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = oc.dot(r.dir);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0. {
            let root = discriminant.sqrt();
            let t = (-half_b - root) / a;
            if t < tmax && t > tmin {
                let p = r.at(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord::new(p, normal, t));
            }

            let t = (-half_b + root) / a;
            if t < tmax && t > tmin {
                let p = r.at(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord::new(p, normal, t));
            }
        }
        None
    }
}
