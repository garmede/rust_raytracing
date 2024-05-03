use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};

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

    pub fn hit(&mut self, r: Ray, t_min: f64, t_max: f64, rec: HitRecord) -> (bool, HitRecord) {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut temp_rec = rec;

        for object in self.objects.iter_mut() {
            let (hit, rec) = object.hit(r, t_min, closest_so_far, temp_rec);
            if hit {
                hit_anything = true;
                closest_so_far = rec.t;
                temp_rec = rec;
            }
        }

        (hit_anything, temp_rec)
    }
}
