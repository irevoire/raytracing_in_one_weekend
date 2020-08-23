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
        if self.hit_sphere(&Sphere::new(Point3::new(0, 0, -1), 0.5)) {
            return Color::new(1, 0, 0);
        }
        let direction = self.dir.unit();
        let t = 0.5 * direction.y + 1.0;
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }

    pub fn hit_sphere(&self, sphere: &Sphere) -> bool {
        let oc = self.orig - sphere.center;
        let a = self.dir.dot(self.dir);
        let b = 2.0 * oc.dot(self.dir);
        let c = oc.dot(oc) - sphere.radius * sphere.radius;
        let discriminant = b * b - 4. * a * c;
        discriminant > 0.
    }
}
