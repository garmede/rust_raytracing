mod sphere;

use raytracing_lib::*;
use std::f64::consts::FRAC_1_SQRT_2;
use std::rc::Rc;

fn main() {
    // 월드
    let mut world = HittableList::new();

    let material_left = Lambertian::new(Vec3(0.0, 0.0, 1.0));
    let material_right = Lambertian::new(Vec3(1.0, 0.0, 0.0));

    world.add(Box::new(sphere::Sphere::new(
        Vec3(-FRAC_1_SQRT_2, 0.0, -1.0),
        FRAC_1_SQRT_2,
        Rc::new(material_left),
    )));

    world.add(Box::new(sphere::Sphere::new(
        Vec3(FRAC_1_SQRT_2, 0.0, -1.0),
        FRAC_1_SQRT_2,
        Rc::new(material_right),
    )));

    // 카메라
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let sampler_per_pixel = 100;
    let max_depth = 50;
    let vfov = 90.0;

    let mut camera = Camera::new(
        aspect_ratio,
        image_width,
        sampler_per_pixel,
        max_depth,
        vfov,
    );

    camera.render(&mut world, "result19.png");
}
