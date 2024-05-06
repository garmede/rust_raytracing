use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        // 히트 레코드의 노말 벡터를 설정합니다.
        // 주의: `매개변수 outward_normal`은 단위 길이를 가정하고 있습니다.

        self.front_face = r.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -(*outward_normal)
        };
    }
}

pub trait Hittable {
    fn hit(&mut self, r: &Ray, t_min: &f64, t_max: &mut f64, rec: &mut HitRecord) -> bool;
}
