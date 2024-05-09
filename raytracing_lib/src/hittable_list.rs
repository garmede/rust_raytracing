use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::interval::*;
use crate::material::*;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&mut self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new(Rc::new(Lambertian::new(Vec3(1.0, 0.0, 1.0))));
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter_mut() {
            if object.hit(r, &Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;

                rec.p = temp_rec.p;
                rec.normal = temp_rec.normal;
                rec.mat = temp_rec.mat.clone();
                rec.t = temp_rec.t;
                rec.front_face = temp_rec.front_face;
            }
        }

        hit_anything
    }
}
