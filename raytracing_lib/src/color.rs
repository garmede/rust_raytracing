use crate::vec3::Vec3;

pub fn write_color(_buf: &mut Vec<u8>, _color: &Vec3) {
    let r = _color.x();
    let g = _color.y();
    let b = _color.z();

    let rbyte = (r * 255.999) as u8;
    let gbyte = (g * 255.999) as u8;
    let bbyte = (b * 255.999) as u8;

    _buf.push(rbyte);
    _buf.push(gbyte);
    _buf.push(bbyte);
}

// pub fn write_color(color: Vec3) -> Vec3 {
//     color * 255.999
// }
