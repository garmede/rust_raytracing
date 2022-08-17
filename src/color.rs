pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            r: (r * 256.0) as u8,
            g: (g * 256.0) as u8,
            b: (b * 256.0) as u8,
        }
    }
}

pub fn out_color(c: Color) -> image::Rgb<u8> {
    image::Rgb([c.r, c.g, c.b])
}
