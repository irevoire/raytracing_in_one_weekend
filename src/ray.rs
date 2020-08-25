use crate::{Color, Hittable, Point3, Vec3};

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

    pub fn color(&self, world: &[&dyn Hittable]) -> Color {
        if let Some(record) = world.hit(self, 0., f64::INFINITY) {
            let target = record.p + record.normal + Point3::random_in_unit_sphere();
            0.5 * Self::new(record.p, target - record.p).color(world)
        } else {
            let unit_direction = self.dir.unit();
            let t = 0.5 * (unit_direction.y + 1.);
            (1.0 - t) * Color::new(1, 1, 1) + t * Color::new(0.5, 0.7, 1)
        }
    }
}
