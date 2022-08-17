use super::vec3::Color;

pub fn out_color(c: Color) -> image::Rgb<u8> {
    let r = (c.x * 256.0) as u8;
    let g = (c.y * 256.0) as u8;
    let b = (c.z * 256.0) as u8;

    image::Rgb([r, g, b])
}
