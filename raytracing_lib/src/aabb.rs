use crate::interval::Interval;
use crate::vec3::Vec3;

pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn new(_x: &Interval, _y: &Interval, _z: &Interval) -> Self {
        Self {
            x: Interval::new(_x.min, _x.max),
            y: Interval::new(_y.min, _y.max),
            z: Interval::new(_z.min, _z.max),
        }
    }

    pub fn new_vec3(a: &Vec3, b: &Vec3) -> Self {
        Self {
            x: if a.0 <= b.0 {
                Interval::new(a.0, b.0)
            } else {
                Interval::new(b.0, a.0)
            },
            y: if a.1 <= b.1 {
                Interval::new(a.1, b.1)
            } else {
                Interval::new(b.1, a.1)
            },
            z: if a.2 <= b.2 {
                Interval::new(a.2, b.2)
            } else {
                Interval::new(b.2, a.2)
            },
        }
    }

    pub fn axis_interval(&self, axis: usize) -> &Interval {
        match axis {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid axis"),
        }
    }

    pub fn hit(&self, r: &crate::ray::Ray, ray_t: Interval) -> bool {
        let ray_origin = r.origin();
        let ray_direction = r.direction();

        let mut _ray_t = ray_t;

        for axis in 0..3 {
            let ax: &Interval = self.axis_interval(axis);
            let adinv = 1.0 / ray_direction[axis];

            let t0 = (ax.min - ray_origin[axis]) * adinv;
            let t1 = (ax.max - ray_origin[axis]) * adinv;

            if t0 < t1 {
                if t0 > _ray_t.min {
                    _ray_t.min = t0;
                }
                if t1 < _ray_t.max {
                    _ray_t.max = t1;
                }
            } else {
                if t1 > _ray_t.min {
                    _ray_t.min = t1;
                }
                if t0 < _ray_t.max {
                    _ray_t.max = t0;
                }
            }

            if _ray_t.max <= _ray_t.min {
                return false;
            }
        }

        true
    }
}
