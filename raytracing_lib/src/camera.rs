use rand::prelude::*;
use std::rc::Rc;

use crate::color::write_color;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::material::*;
use crate::ray::Ray;
use crate::vec3::*;

pub struct Camera {
    pub aspect_ratio: f64,     // 이미지 너비와 높이의 비율
    pub image_width: u32,      // 렌더링될 이미지 너비(픽셀 수)
    pub sample_per_pixel: u32, // 각 픽셀의 무작위 샘플 수
    pub max_depth: u32,        // 재귀 깊이
    pub vfov: f64,             // 수직 시야각
    pub lookfrom: Vec3,        // 카메라 위치
    pub lookat: Vec3,          // 카메라가 바라보는 위치
    pub vup: Vec3,             // 카메라의 상단 방향
    image_height: u32,         // 이미지 높이
    pixel_samples_scale: f64,  // 픽셀 샘플 합계에 대한 색상 배율
    center: Vec3,              // 카메라 중심
    pixel00_loc: Vec3,         // 0,0 픽셀 위치
    pixel_delta_u: Vec3,       // 오른쪽으로 오프셋될 픽셀
    pixel_delta_v: Vec3,       // 아래로 오프셋될 픽셀
    u: Vec3,                   // 카메라의 수평 방향
    v: Vec3,                   // 카메라의 수직 방향
    w: Vec3,                   // 카메라의 전방 방향
    image_buf: Vec<u8>,        // 이미지 버퍼
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        sample_per_pixel: u32,
        max_depth: u32,
        vfov: f64,
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
    ) -> Self {
        Self {
            aspect_ratio,
            image_width,
            sample_per_pixel,
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
            image_height: 0,
            pixel_samples_scale: 0.0,
            center: Vec3(0.0, 0.0, 0.0),
            pixel00_loc: Vec3(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3(0.0, 0.0, 0.0),
            u: Vec3(0.0, 0.0, 0.0),
            v: Vec3(0.0, 0.0, 0.0),
            w: Vec3(0.0, 0.0, 0.0),
            image_buf: vec![],
        }
    }

    pub fn render(&mut self, world: &mut HittableList, path: &str) {
        let mut writer = self.initialize(path);

        for y in 0..self.image_height {
            eprint!("\r남은 스캔라인: {}            ", self.image_height - y);
            for x in 0..self.image_width {
                let mut pixel_color = Vec3(0.0, 0.0, 0.0);
                for _ in 0..self.sample_per_pixel {
                    let r = self.get_ray(x, y);
                    pixel_color += Self::ray_color(&r, self.max_depth, world);
                }
                pixel_color *= self.pixel_samples_scale;
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

        self.pixel_samples_scale = 1.0 / self.sample_per_pixel as f64;

        self.center = self.lookfrom;

        // 뷰포트 크기를 결정합니다.
        let focal_length = (self.lookfrom - self.lookat).length();
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // 카메라 좌표계의 u,v,w 단위 기저 벡터를 계산합니다.
        self.w = (self.lookfrom - self.lookat).unit_vector();
        self.u = cross(&self.vup, &self.w).unit_vector();
        self.v = cross(&self.w, &self.u);

        // 수평 및 수직 뷰포트 가장자리 아래의 벡터를 계산합니다.
        let viewport_u = self.u * viewport_width;
        let viewport_v = -self.v * viewport_height;

        // 픽셀 간 수평 및 수직 델타 벡터를 계산합니다.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // 좌측 상단 픽셀의 위치를 ​​계산합니다.
        let viewport_upper_left =
            self.center - (self.w * focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;

        let file = std::fs::File::create(path).unwrap();
        let mut encoder = png::Encoder::new(file, self.image_width, self.image_height);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);

        encoder.write_header().unwrap()
    }

    fn ray_color(r: &Ray, depth: u32, world: &mut HittableList) -> Vec3 {
        if depth <= 0 {
            return Vec3(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::new(Rc::new(Lambertian::new(Vec3(1.0, 0.0, 1.0))));

        if world.hit(r, &Interval::new(0.001, f64::INFINITY), &mut rec) {
            let mut scattered = Ray::new(Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 0.0));
            let mut attenuation = Vec3(0.0, 0.0, 0.0);

            if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * Self::ray_color(&scattered, depth - 1, world);
            }

            return Vec3(0.0, 0.0, 0.0);
        }

        // 배경 그라디언트
        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);

        Vec3(1.0, 1.0, 1.0) * (1.0 - a) + Vec3(0.5, 0.7, 1.0) * a
    }

    fn get_ray(&self, x: u32, y: u32) -> Ray {
        // 원점에서 시작하고 픽셀 위치 x, y 주변에서 무작위로 샘플링된 지점을 향하는 카메라 레이를 생성합니다.
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (offset.x() + x as f64))
            + (self.pixel_delta_v * (offset.y() + y as f64));
        let ray_direction = pixel_sample - self.center;

        Ray::new(self.center, ray_direction)
    }

    fn sample_square() -> Vec3 {
        // [-.5,-.5]-[+.5,+.5] 단위 사각형 내의 무작위 점에 대한 벡터를 반환합니다.
        Vec3(random::<f64>() - 0.5, random::<f64>() - 0.5, 0.0)
    }
}
