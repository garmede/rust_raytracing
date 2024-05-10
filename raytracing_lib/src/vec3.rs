use rand::prelude::*;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub};

#[derive(Copy, Clone)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn near_zero(&self) -> bool {
        const S: f64 = 1e-8;
        self.0.abs() < S && self.1.abs() < S && self.2.abs() < S
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
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

pub fn random_double() -> f64 {
    random::<f64>()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    thread_rng().gen_range(min..max)
}

pub fn random_vec3() -> Vec3 {
    Vec3(random::<f64>(), random::<f64>(), random::<f64>())
}

pub fn random_vec3_range(min: f64, max: f64) -> Vec3 {
    Vec3(
        thread_rng().gen_range(min..max),
        thread_rng().gen_range(min..max),
        thread_rng().gen_range(min..max),
    )
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_vec3_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3(
            thread_rng().gen_range(-1.0..1.0),
            thread_rng().gen_range(-1.0..1.0),
            0.0,
        );
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit_vector()
}

pub fn random_on_hamisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if dot(&on_unit_sphere, normal) > 0.0 {
        return on_unit_sphere;
    } else {
        return -on_unit_sphere;
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - *n * 2.0 * dot(v, n)
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(&-*uv, n).min(1.0);
    let r_out_perp = (*uv + *n * cos_theta) * etai_over_etat;
    let r_out_parallel = *n * -(1.0 - r_out_perp.length_squared()).abs().sqrt();
    r_out_perp + r_out_parallel
}

pub fn dot(a: &Vec3, b: &Vec3) -> f64 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

pub fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
    Vec3(
        a.1 * b.2 - a.2 * b.1,
        a.2 * b.0 - a.0 * b.2,
        a.0 * b.1 - a.1 * b.0,
    )
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

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
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

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, t: Self) {
        self.0 *= t.0;
        self.1 *= t.1;
        self.2 *= t.2;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.0 *= t;
        self.1 *= t;
        self.2 *= t;
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Vec3 {
        Vec3(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Vec3 {
        Vec3(self.0 / t, self.1 / t, self.2 / t)
    }
}
