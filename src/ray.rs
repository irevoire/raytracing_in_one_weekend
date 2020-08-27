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
        self.orig + t * self.dir
    }

    pub fn color(&self, world: &[&dyn Hittable], depth: usize) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth == 0 {
            return Color::default();
        }

        if let Some(record) = world.hit(self, 0.001, f64::INFINITY) {
            let mut attenuation = Color::default();
            if let Some(scattered) = record.material.scatter(self, &record, &mut attenuation) {
                attenuation * scattered.color(world, depth - 1)
            } else {
                Color::default()
            }
        } else {
            let unit_direction = self.dir.unit();
            let t = 0.5 * (unit_direction.y + 1.);
            (1.0 - t) * Color::new(1, 1, 1) + t * Color::new(0.5, 0.7, 1)
        }
    }
}
