use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

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
    fn hit(&mut self, r: &Ray, t_min: &f64, t_max: &mut f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;

        for object in self.objects.iter_mut() {
            if object.hit(r, t_min, t_max, rec) {
                hit_anything = true;
                *t_max = rec.t;
            }
        }

        hit_anything
    }
}
