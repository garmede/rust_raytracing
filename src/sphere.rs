use raytracing_lib::*;
use std::rc::Rc;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&mut self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - r.origin();
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
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat = Rc::clone(&self.mat);

        true
    }
}
