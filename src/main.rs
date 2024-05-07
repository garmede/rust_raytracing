// use image::ImageBuffer;
use raytracing_lib::hittable::{HitRecord, Hittable};
use raytracing_lib::hittable_list::HittableList;
use raytracing_lib::ray::Ray;
use raytracing_lib::vec3::Vec3;
mod sphere;
use sphere::Sphere;

fn main() {
    // 이미지 높이를 계산하고 1 이상인지 확인.
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT_ASPECT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const IMAGE_HEIGHT: u32 = if IMAGE_HEIGHT_ASPECT < 1 {
        1
    } else {
        IMAGE_HEIGHT_ASPECT
    };
    const IMAGE_PIXELS: usize = IMAGE_WIDTH as usize * IMAGE_HEIGHT as usize * 3;
    let height_digit = IMAGE_HEIGHT.to_string().len();

    let mut world = HittableList::new();
    world.add(Box::new(Sphere {
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.5,
    }));
    world.add(Box::new(Sphere {
        center: Vec3(0.0, -100.5, -1.0),
        radius: 100.0,
    }));

    // 카메라
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);
    let camera_center = Vec3(0.0, 0.0, 0.0);

    // 수평 뷰포트 가장자리를 가로질러 벡터를 계산하고, 수직 뷰포트 가장자리를 따라 아래로 벡터를 계산.
    let viewport_u = Vec3(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3(0.0, -viewport_height, 0.0);

    // 픽셀에서 픽셀로의 수평 및 수직 델타 벡터를 계산.
    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f64;

    // 왼쪽 상단 픽셀의 위치를 ​​계산.
    let viewport_upper_left =
        camera_center - Vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.0;

    // let mut imgbuf = ImageBuffer::new(image_width, image_height);

    let mut writer = set_encoder("result1.png", IMAGE_WIDTH, IMAGE_HEIGHT);
    let mut imgbuf = [0u8; IMAGE_PIXELS];

    for y in 0..IMAGE_HEIGHT {
        eprint!("\r남은 스캔라인: {:0height_digit$}", IMAGE_HEIGHT - y);
        for x in 0..IMAGE_WIDTH {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * x as f64) + (pixel_delta_v * y as f64);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r, &mut world);

            // let pixel = imgbuf.get_pixel_mut(x, y);
            // *pixel = to_rgb(raytracing_lib::color::write_color(pixel_color))

            let color = raytracing_lib::color::write_color(pixel_color);
            write_buffer(&mut imgbuf, &color, x, y, IMAGE_WIDTH);
        }
    }
    eprintln!("\r완료!                                ");
    writer.write_image_data(&imgbuf).unwrap();
    // imgbuf.save("result2.png").unwrap();
}

fn set_encoder(path: &str, width: u32, height: u32) -> png::Writer<std::fs::File> {
    let file = std::fs::File::create(path).unwrap();
    let mut encoder = png::Encoder::new(file, width, height);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.write_header().unwrap()
}

fn write_buffer(_buf: &mut [u8], _color: &Vec3, x: u32, y: u32, width: u32) {
    let index = (y * width + x) as usize * 3;
    _buf[index] = _color.x() as u8;
    _buf[index + 1] = _color.y() as u8;
    _buf[index + 2] = _color.z() as u8;
}

// fn to_rgb(color: Vec3) -> image::Rgb<u8> {
//     image::Rgb([color.x() as u8, color.y() as u8, color.z() as u8])
// }

fn ray_color(r: &Ray, world: &mut HittableList) -> Vec3 {
    let mut rec = HitRecord::new();
    let zero = 0.0;
    let mut infinity = f64::INFINITY;
    if world.hit(r, &zero, &mut infinity, &mut rec) {
        return (rec.normal + Vec3(1.0, 1.0, 1.0)) * 0.5;
    }

    // 배경 그라디언트
    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    Vec3(1.0, 1.0, 1.0) * (1.0 - a) + Vec3(0.5, 0.7, 1.0) * a
}
