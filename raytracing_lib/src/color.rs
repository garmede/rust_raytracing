use crate::interval::Interval;
use crate::vec3::Vec3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component < 0.0 {
        0.0
    } else {
        linear_component.sqrt()
    }
}

pub fn write_color(_buf: &mut Vec<u8>, _color: &Vec3) {
    let r = linear_to_gamma(_color.x());
    let g = linear_to_gamma(_color.y());
    let b = linear_to_gamma(_color.z());

    const INTENSITY: Interval = Interval { min: 0.0, max: 0.999 };

    let rbyte = (INTENSITY.clamp(r) * 256.0) as u8;
    let gbyte = (INTENSITY.clamp(g) * 256.0) as u8;
    let bbyte = (INTENSITY.clamp(b) * 256.0) as u8;

    _buf.push(rbyte);
    _buf.push(gbyte);
    _buf.push(bbyte);
}
