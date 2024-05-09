mod sphere;

use raytracing_lib::*;
use std::rc::Rc;

fn main() {
    // 월드
    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian::new(Vec3(0.5, 0.5, 0.5)));
    world.add(Box::new(sphere::Sphere::new(
        Vec3(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Vec3(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = random_vec3() * random_vec3();
                    Rc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_vec3_range(0.5, 1.0);
                    let fuzz = random_double() * 0.5;
                    Rc::new(Metal::new(albedo, fuzz))
                } else {
                    // glass
                    Rc::new(Dielectric::new(1.5))
                };

                world.add(Box::new(sphere::Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Box::new(sphere::Sphere::new(
        Vec3(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(Vec3(0.4, 0.2, 0.1)));
    world.add(Box::new(sphere::Sphere::new(
        Vec3(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Vec3(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(sphere::Sphere::new(
        Vec3(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    // 카메라
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let sampler_per_pixel = 500;
    let max_depth = 50;

    let vfov = 20.0;
    let lookfrom = Vec3(13.0, 2.0, 3.0);
    let lookat = Vec3(0.0, 0.0, 0.0);
    let vup = Vec3(0.0, 1.0, 0.0);

    let defocus_dist = 0.6;
    let focus_dist = 10.0;

    let mut camera = Camera::new(
        aspect_ratio,
        image_width,
        sampler_per_pixel,
        max_depth,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_dist,
        focus_dist,
    );

    camera.render(&mut world, "result23.png");
}
