use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }

    pub fn set_x(&mut self, _x: f64) {
        self.0 = _x;
    }

    pub fn set_y(&mut self, _y: f64) {
        self.1 = _y;
    }

    pub fn set_z(&mut self, _z: f64) {
        self.2 = _z;
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }
}

pub fn dot(a: &Vec3, b: &Vec3) -> f64 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, other: f64) -> Vec3 {
        Vec3(self.0 + other, self.1 + other, self.2 + other)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, t: f64) -> Vec3 {
        Vec3(self.0 - t, self.1 - t, self.2 - t)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Vec3 {
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3(self.0 * t, self.1 * t, self.2 * t)
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Vec3 {
        Vec3(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        Vec3(self.0 / t, self.1 / t, self.2 / t)
    }
}
