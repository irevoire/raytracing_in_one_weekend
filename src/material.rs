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
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: impl Into<f64>) -> Self {
        Self {
            albedo,
            fuzz: fuzz.into().clamp(0., 1.),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color) -> Option<Ray> {
        let reflected = r_in.dir.unit().reflect(rec.normal);
        *attenuation = self.albedo;
        let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());

        if scattered.dir.dot(rec.normal) > 0. {
            Some(scattered)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color) -> Option<Ray> {
        *attenuation = Color::new(1, 1, 1);
        let etai_over_etat = match rec.front_face {
            true => 1. / self.ref_idx,
            false => self.ref_idx,
        };
        let unit_direction = r_in.dir.unit();

        let cos_theta = (-unit_direction).dot(rec.normal).min(1.);
        let sin_theta = (1. - (cos_theta * cos_theta)).sqrt();

        if etai_over_etat * sin_theta > 1. {
            // must reflect
            let reflected = unit_direction.reflect(rec.normal);
            Some(Ray::new(rec.p, reflected))
        } else {
            // can refract
            let refracted = unit_direction.refract(rec.normal, etai_over_etat);
            Some(Ray::new(rec.p, refracted))
        }
    }
}
