use raytracing_lib::{camera::Camera, hittable_list::HittableList, material::*, vec3::Vec3};
use std::rc::Rc;
mod sphere;
use sphere::Sphere;

fn main() {
    // 월드
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Vec3(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Vec3(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_bubble = Dielectric::new(1.0 / 1.5);
    let material_right = Metal::new(Vec3(0.8, 0.6, 0.2), 1.0);

    world.add(Box::new(Sphere::new(
        Vec3(0.0, -100.5, -1.0),
        100.0,
        Rc::new(material_ground),
    )));
    world.add(Box::new(Sphere::new(
        Vec3(0.0, 0.0, -1.2),
        0.5,
        Rc::new(material_center),
    )));
    world.add(Box::new(Sphere::new(
        Vec3(-1.0, 0.0, -1.0),
        0.5,
        Rc::new(material_left),
    )));
    world.add(Box::new(Sphere::new(
        Vec3(-1.0, 0.0, -1.0),
        0.4,
        Rc::new(material_bubble),
    )));
    world.add(Box::new(Sphere::new(
        Vec3(1.0, 0.0, -1.0),
        0.5,
        Rc::new(material_right),
    )));

    // 카메라
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let sampler_per_pixel = 100;
    let max_depth = 50;
    let mut camera = Camera::new(aspect_ratio, image_width, sampler_per_pixel, max_depth);

    camera.render(&mut world, "result18.png");
}
