use crate::interval::Interval;
use crate::vec3::Vec3;

pub fn write_color(_buf: &mut Vec<u8>, _color: &Vec3) {
    let r = _color.x();
    let g = _color.y();
    let b = _color.z();

    const INTENSITY: Interval = Interval { min: 0.0, max: 1.0 };

    let rbyte = (INTENSITY.clamp(r) * 256.0) as u8;
    let gbyte = (INTENSITY.clamp(g) * 256.0) as u8;
    let bbyte = (INTENSITY.clamp(b) * 256.0) as u8;

    _buf.push(rbyte);
    _buf.push(gbyte);
    _buf.push(bbyte);
}
