use image::ImageBuffer;
use raytracing_lib::hittable::{HitRecord, Hittable};
use raytracing_lib::hittable_list::HittableList;
use raytracing_lib::ray::Ray;
use raytracing_lib::vec3::Vec3;
mod sphere;
use sphere::Sphere;

fn main() {
    // 이미지 높이를 계산하고 1 이상인지 확인.
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height_aspect = (image_width as f64 / aspect_ratio) as u32;
    let image_height = if image_height_aspect < 1 {
        1
    } else {
        image_height_aspect
    };
    let height_digit = image_height.to_string().len();

    let mut world = HittableList::new();
    world.add(Box::new(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    }));
    world.add(Box::new(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    }));

    // 카메라
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    // 수평 뷰포트 가장자리를 가로질러 벡터를 계산하고, 수직 뷰포트 가장자리를 따라 아래로 벡터를 계산.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // 픽셀에서 픽셀로의 수평 및 수직 델타 벡터를 계산.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // 왼쪽 상단 픽셀의 위치를 ​​계산.
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.0;

    let mut imgbuf = ImageBuffer::new(image_width, image_height);

    for y in 0..image_height {
        eprint!("\r남은 스캔라인: {:0height_digit$}", image_height - y);
        for x in 0..image_width {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * x as f64) + (pixel_delta_v * y as f64);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r, &mut world);

            let pixel = imgbuf.get_pixel_mut(x, y);

            *pixel = to_rgb(raytracing_lib::color::write_color(pixel_color))
        }
    }
    eprintln!("\r완료!                                ");
    imgbuf.save("result2.png").unwrap();
}

fn to_rgb(color: Vec3) -> image::Rgb<u8> {
    image::Rgb([color.x as u8, color.y as u8, color.z as u8])
}

fn ray_color(r: &Ray, world: &mut HittableList) -> Vec3 {
    let mut rec = HitRecord::new();
    let zero = 0.0;
    let mut infinity = f64::INFINITY;
    if world.hit(r, &zero, &mut infinity, &mut rec) {
        return (rec.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
    }

    // 배경 그라디언트
    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
}
