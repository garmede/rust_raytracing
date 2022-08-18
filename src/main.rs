use std::io::{self, Write};

mod color;
mod ray;
mod vec3;

use ray::Ray;
use vec3::Color;
use vec3::Point3;
use vec3::Vec3;

fn ray_color(r: &Ray) -> Color {
    let mut t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let N = vec3::normalize(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return 0.5 * Color::new(N.x + 1.0, N.y + 1.0, N.z + 1.0);
    }
    let unit_direction = vec3::normalize(r.dir);
    t = 0.5 * (unit_direction.y + 1.0);
    vec3::lerp(Color::new(1.0, 1.0, 1.0), Color::new(0.5, 0.7, 1.0), t)
}

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.orig - center;
    let a = r.dir.length_squared();
    let half_b = vec3::dot(oc, r.dir);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
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
            *pixel = color::out_color(ray_color(&ray));
        }
    }

    imgbuf.save("render/04_normal_sphere.png").unwrap();
    print!("\nDone.\n");
}
