use crate::{Color, Point3, Sphere, Vec3};

#[derive(Debug)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }

    pub fn color(&self) -> Color {
        if let Some(t) = self.hit_sphere(&Sphere::new(Point3::new(0, 0, -1), 0.5)) {
            let N = self.at(t) - Vec3::new(0, 0, -1);
            0.5 * (N + 1.)
        } else {
            let direction = self.dir.unit();
            let t = 0.5 * direction.y + 1.0;
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }

    pub fn hit_sphere(&self, sphere: &Sphere) -> Option<f64> {
        let oc = self.orig - sphere.center;
        let a = self.dir.dot(self.dir);
        let b = 2.0 * oc.dot(self.dir);
        let c = oc.dot(oc) - sphere.radius * sphere.radius;
        let discriminant = b * b - 4. * a * c;
        if discriminant < 0. {
            None
        } else {
            Some((-b - discriminant.sqrt()) / (2. * a))
        }
    }
}
