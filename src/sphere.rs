use crate::{HitRecord, Hittable, Material, Point3, Ray};

#[derive(Clone)]
pub struct Sphere<'a> {
    pub center: Point3,
    pub radius: f64,
    pub material: &'a dyn Material,
}

impl<'a> Sphere<'a> {
    pub fn new(center: Point3, radius: f64, material: &'a dyn Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<'a> Hittable for Sphere<'a> {
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
                let outward_normal = (p - self.center) / self.radius;
                let mut rec = HitRecord::new(p, normal, self.material, t, false);
                rec.set_face_normal(r, outward_normal);
                return Some(rec);
            }

            let t = (-half_b + root) / a;
            if t < tmax && t > tmin {
                let p = r.at(t);
                let normal = (p - self.center) / self.radius;
                let outward_normal = (p - self.center) / self.radius;
                let mut rec = HitRecord::new(p, normal, self.material, t, false);
                rec.set_face_normal(r, outward_normal);
                return Some(rec);
            }
        }
        None
    }
}
