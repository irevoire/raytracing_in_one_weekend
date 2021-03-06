pub mod hittable;
pub(crate) mod rand;
pub use hittable::{HitRecord, Hittable};
pub mod material;
pub use material::*;
mod ray;
pub use ray::Ray;
mod sphere;
pub use sphere::Sphere;
mod vec3;
pub use vec3::Vec3;
mod camera;
pub use camera::Camera;

pub type Point3 = Vec3;
pub type Color = Vec3;
pub type World = Vec<&'static dyn Hittable>;
