mod sphere;

use raytracing_lib::*;
use std::rc::Rc;

fn main() {
    // 월드
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Vec3(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Vec3(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_bubble = Rc::new(Dielectric::new(1.0 / 1.5));
    let material_right = Rc::new(Metal::new(Vec3(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(sphere::Sphere::new(
        Vec3(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));

    world.add(Box::new(sphere::Sphere::new(
        Vec3(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));

    world.add(Box::new(sphere::Sphere::new(
        Vec3(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));

    world.add(Box::new(sphere::Sphere::new(
        Vec3(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    )));

    world.add(Box::new(sphere::Sphere::new(
        Vec3(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // 카메라
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let sampler_per_pixel = 100;
    let max_depth = 50;
    let vfov = 20.0;
    let lookfrom = Vec3(-2.0, 2.0, 1.0);
    let lookat = Vec3(0.0, 0.0, -1.0);
    let vup = Vec3(0.0, 1.0, 0.0);

    let mut camera = Camera::new(
        aspect_ratio,
        image_width,
        sampler_per_pixel,
        max_depth,
        vfov,
        lookfrom,
        lookat,
        vup,
    );

    camera.render(&mut world, "result21.png");
}
