use std::sync::Arc;

use raytracer::{
    geometry::sphere::Sphere,
    util::{print_color, ray_color},
    vec::Vector3,
    vec3,
    view::ray::{HitTarget, Ray},
};

fn main() {
    let aspect_ratio = 16. / 9.;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    let viewport_height = 2.;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.;

    let origin = Vector3::zero();
    let horizontal = Vector3::new(viewport_width, 0, 0);
    let vertical = Vector3::new(0, viewport_height, 0);
    let lower_left_corner =
        origin - horizontal / 2 - vertical / 2 - Vector3::new(0, 0, focal_length);

    let mut world = HitTarget::new();
    (*world).push(Arc::new(Sphere::new(vec3![0, 0, -1], 0.5)));
    (*world).push(Arc::new(Sphere::new(vec3![0, -100.5, -1], 100.)));

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);

        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let ray = Ray::of(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&ray, &world);
            print_color(pixel_color);
        }
    }
}
