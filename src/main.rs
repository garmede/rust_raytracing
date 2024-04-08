// mod color;
// mod ray;
// mod vec3;

use image::ImageBuffer;
use rayracing::Ray;
use rayracing::Vec3;

fn main() {
    // 이미지 높이를 계산하고 1 이상인지 확인.
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height_aspect = (image_width as f32 / aspect_ratio) as u32;
    let image_height = if image_height_aspect < 1 {
        1
    } else {
        image_height_aspect
    };
    let height_digit = image_height.to_string().len();

    // 카메라
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    // 수평 뷰포트 가장자리를 가로질러 벡터를 계산하고, 수직 뷰포트 가장자리를 따라 아래로 벡터를 계산.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // 픽셀에서 픽셀로의 수평 및 수직 델타 벡터를 계산.
    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    // 왼쪽 상단 픽셀의 위치를 ​​계산.
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.0;

    let mut imgbuf = ImageBuffer::new(image_width, image_height);

    for y in 0..image_height {
        eprint!("\r남은 스캔라인: {:0height_digit$}", image_height - y);
        for x in 0..image_width {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * x as f32) + (pixel_delta_v * y as f32);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r);

            let pixel = imgbuf.get_pixel_mut(x, y);

            *pixel = color::write_color(pixel_color);
        }
    }
    eprintln!("\r완료!                                ");
    imgbuf.save("result.png").unwrap();
}

fn ray_color(r: Ray) -> Vec3 {
    let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r);
    let ray = Ray::new(r.origin(), r.direction());
    if t > 0.0 {
        let n = Vec3::unit_vector(ray.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) / 2.0;
    }

    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn hit_sphere(center: Vec3, radius: f32, r: Ray) -> f32 {
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = oc.dot(r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    return if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    };
}
