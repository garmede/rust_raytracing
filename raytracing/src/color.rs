use crate::vec3::Vec3;
use image::Rgb;

pub fn write_color(color: Vec3) -> Rgb<u8> {
    let r = (255.999 * color.x()) as u8;
    let g = (255.999 * color.y()) as u8;
    let b = (255.999 * color.z()) as u8;

    Rgb([r, g, b])
}
