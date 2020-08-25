use rand::Rng;

#[derive(Debug, Copy, Clone, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    /// return a random generated vector between 0 and 1
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen(),
        }
    }

    /// return a random generated vector in the specified range
    /// Panic if low >= high
    pub fn random_range(low: impl Into<f64>, high: impl Into<f64>) -> Self {
        let (low, high) = (low.into(), high.into());
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(low, high),
            y: rng.gen_range(low, high),
            z: rng.gen_range(low, high),
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let v = Self::random_range(-1, 1);
            if v.length_squared() < 1. {
                return v;
            }
        }
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn unit(self) -> Self {
        self / 3.0
    }

    pub fn color(self, samples_per_pixel: usize) -> String {
        let scale = 1. / samples_per_pixel as f64;

        // Divide the color by the number of samples and gamma-correct for gamma=2.0.
        let r = (self.x * scale).sqrt();
        let g = (self.y * scale).sqrt();
        let b = (self.z * scale).sqrt();

        // Write the translated [0,255] value of each color component.
        format!(
            "{} {} {}",
            (256. * r.clamp(0., 0.999)) as u64,
            (256. * g.clamp(0., 0.999)) as u64,
            (256. * b.clamp(0., 0.999)) as u64,
        )
    }
}

use std::ops::*;

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, other: f64) -> Self::Output {
        Self::new(self.x + other, self.y + other, self.z + other)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Self::new(self.x * other, self.y * other, self.z * other)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3::new(self * other.x, self * other.y, self * other.z)
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        Self::new(self.x / other, self.y / other, self.z / other)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}
