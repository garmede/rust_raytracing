use std::io::{self, Write};

mod color;
// mod vec3;

use color::Color;
// use vec3::Vec3;

fn main() {
    // Image
    let image_width = 256;
    let image_height = 256;

    // Render
    let mut imgbuf = image::ImageBuffer::new(image_width, image_height);

    for j in (0..image_height).rev() {
        print!("\rScanlines remaining: {j}");
        io::stdout().flush().unwrap();
        for i in 0..image_width {
            let pixel_color =
                Color::new(i as f64 / image_width as f64, j as f64 / image_height as f64, 0.25);

            let pixel = imgbuf.get_pixel_mut(i, image_height - 1 - j);
            *pixel = color::out_color(pixel_color);
        }
    }

    imgbuf.save("render/first_image.png").unwrap();
    print!("\nDone.\n");
}
