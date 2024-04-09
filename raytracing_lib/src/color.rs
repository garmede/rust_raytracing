use crate::vec3::Vec3;
// use image::Rgb;

pub fn write_color(color: Vec3) -> Vec3 {
    color * 255.999
}
