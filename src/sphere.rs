use raytracing_lib::*;
use std::rc::Rc;

pub struct Sphere {
    center1: Vec3,
    radius: f64,
    mat: Rc<dyn Material>,
    is_moving: bool,
    center_vec: Vec3,
}

impl Sphere {
    pub fn new(center1: Vec3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Self {
            center1,
            radius: radius.max(0.0),
            mat,
            is_moving: false,
            center_vec: center1,
        }
    }

    pub fn new_moving(center1: Vec3, center2: Vec3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Self {
            center1,
            radius: radius.max(0.0),
            mat,
            is_moving: true,
            center_vec: center2 - center1,
        }
    }

    fn sphere_center(&self, time: f64) -> Vec3 {
        self.center1 + self.center_vec * time
    }
}

impl Hittable for Sphere {
    fn hit(&mut self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let center = if self.is_moving {
            Self::sphere_center(self, r.time())
        } else {
            self.center1
        };
        let oc = center - r.origin();
        let a = r.direction().length_squared();
        let h = dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat = Rc::clone(&self.mat);

        true
    }
}
