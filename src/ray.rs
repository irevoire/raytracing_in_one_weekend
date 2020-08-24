use crate::{Color, Hittable, Point3, Sphere, Vec3};

#[derive(Debug, Clone)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }

    pub fn color(&self) -> Color {
        let sphere = Sphere::new(Point3::new(0, 0, -1), 0.5);
        if let Some(record) = sphere.hit(self, 0., 100.) {
            let N = self.at(record.t) - Vec3::new(0, 0, -1);
            0.5 * (N + 1.)
        } else {
            let direction = self.dir.unit();
            let t = 0.5 * direction.y + 1.0;
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }
}
