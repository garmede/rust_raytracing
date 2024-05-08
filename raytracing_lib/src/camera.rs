use crate::color::write_color;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    pub aspect_ratio: f64, // 종횡비
    pub image_width: u32,  // 이미지 너비
    image_height: u32,     // 이미지 높이
    center: Vec3,          // 카메라 중심
    pixel00_loc: Vec3,     // 0,0 픽셀 위치
    pixel_delta_u: Vec3,   // 오른쪽으로 오프셋될 픽셀
    pixel_delta_v: Vec3,   // 아래로 오프셋될 픽셀
    image_buf: Vec<u8>,    // 이미지 버퍼
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32) -> Self {
        Self {
            aspect_ratio,
            image_width,
            image_height: 0,
            center: Vec3(0.0, 0.0, 0.0),
            pixel00_loc: Vec3(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3(0.0, 0.0, 0.0),
            image_buf: vec![],
        }
    }

    pub fn render(&mut self, world: &mut HittableList, path: &str) {
        let mut writer = self.initialize(path);

        fn ray_color(r: &Ray, world: &mut HittableList) -> Vec3 {
            let mut rec = HitRecord::new();
            if world.hit(r, &Interval::new(0.0, f64::INFINITY), &mut rec) {
                return (rec.normal + Vec3(1.0, 1.0, 1.0)) * 0.5;
            }

            // 배경 그라디언트
            let unit_direction = r.direction().unit_vector();
            let a = 0.5 * (unit_direction.y() + 1.0);

            Vec3(1.0, 1.0, 1.0) * (1.0 - a) + Vec3(0.5, 0.7, 1.0) * a
        }

        for y in 0..self.image_height {
            eprint!("\r남은 스캔라인: {}", self.image_height - y);
            for x in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (self.pixel_delta_u * x as f64)
                    + (self.pixel_delta_v * y as f64);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);

                let pixel_color = ray_color(&r, world);
                write_color(&mut self.image_buf, &pixel_color);
            }
        }

        eprintln!("\r완료!                                ");
        writer.write_image_data(&self.image_buf).unwrap();
    }

    fn initialize(&mut self, path: &str) -> png::Writer<std::fs::File> {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;

        if self.image_height < 1 {
            self.image_height = 1;
        }

        self.center = Vec3(0.0, 0.0, 0.0);

        // 뷰포트 크기를 결정합니다.
        const FOCAL_LENGTH: f64 = 1.0;
        const VIEWPORT_HEIGHT: f64 = 2.0;
        let viewport_width = VIEWPORT_HEIGHT * (self.image_width as f64 / self.image_height as f64);

        // 수평 및 수직 뷰포트 가장자리 아래의 벡터를 계산합니다.
        let viewport_u = Vec3(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3(0.0, -VIEWPORT_HEIGHT, 0.0);

        // 픽셀 간 수평 및 수직 델타 벡터를 계산합니다.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // 좌측 상단 픽셀의 위치를 ​​계산합니다.
        let viewport_upper_left =
            self.center - Vec3(0.0, 0.0, FOCAL_LENGTH) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;

        let file = std::fs::File::create(path).unwrap();
        let mut encoder = png::Encoder::new(file, self.image_width, self.image_height);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.write_header().unwrap()
    }
}
