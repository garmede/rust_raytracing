use raytracing_lib::hittable;

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&mut self, r: Ray, t_min: f64, t_max: f64, rec: HitRecord) -> (bool, HitRecord) {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return (false, rec);
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return (false, rec);
            }
        }

        let mut new_rec = HitRecord {
            p: r.at(rec.t),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: root,
            front_face: false,
        };

        let outward_normal = (new_rec.p - self.center) / self.radius;
        new_rec.set_face_normal(r, outward_normal);

        new_rec.normal = (new_rec.p - self.center) / self.radius;

        (true, new_rec)
    }
}
