use std::io::{self, Write};

mod color;
mod ray;
mod vec3;

use ray::Ray;
use vec3::Color;
use vec3::Point3;
use vec3::Vec3;

fn ray_color(r: Ray) -> Color {
    let unit_direction = vec3::normalize(r.dir);
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    let mut imgbuf = image::ImageBuffer::new(image_width, image_height);

    for j in (0..image_height).rev() {
        print!("\rScanlines remaining: {j}");
        io::stdout().flush().unwrap();
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let pixel = imgbuf.get_pixel_mut(i, image_height - 1 - j);
            *pixel = color::out_color(ray_color(ray));
        }
    }

    imgbuf.save("render/02_gradient.png").unwrap();
    print!("\nDone.\n");
}
