use crate::*;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color) -> Option<Ray>;
}

impl std::fmt::Debug for dyn Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Material")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Color) -> Option<Ray> {
        let scatter_direction = rec.normal + Vec3::random_unit_vector();
        *attenuation = self.albedo;

        Some(Ray::new(rec.p, scatter_direction))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color) -> Option<Ray> {
        let reflected = r_in.dir.unit().reflect(rec.normal);
        *attenuation = self.albedo;
        let scattered = Ray::new(rec.p, reflected);

        if scattered.dir.dot(rec.normal) > 0. {
            Some(scattered)
        } else {
            None
        }
    }
}
