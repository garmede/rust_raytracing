use super::vec3::{Point3, Vec3};

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            orig: origin,
            dir: direction,
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        (self.orig + t * self.dir) as Point3
    }
}
