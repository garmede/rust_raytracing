use raytracing_lib::{
    camera::Camera, hittable_list::HittableList, vec3::Vec3,
};
mod sphere;
use sphere::Sphere;

fn main() {
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
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let sampler_per_pixel = 100;
    let max_depth = 50;
    let mut camera = Camera::new(aspect_ratio, image_width, sampler_per_pixel, max_depth);

    camera.render(&mut world, "result10.png");
}
