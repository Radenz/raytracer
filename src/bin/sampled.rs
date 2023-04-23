use std::{sync::Arc, time::Instant};

use image::{ImageBuffer, Rgb};
use raytracer::{
    object::{
        geometry::{sphere::Sphere, vector::Vector3},
        material::color::Color,
    },
    render::pixel::Vector3Extension,
    util::{random::Random, ray_color},
    vec3,
    view::{camera::Camera, ray::HitTarget},
};
fn main() {
    let aspect_ratio = 16. / 9.;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples = 100;

    let mut world = HitTarget::new();
    (*world).push(Arc::new(Sphere::new(vec3![0, 0, -1], 0.5)));
    (*world).push(Arc::new(Sphere::new(vec3![0, -100.5, -1], 100.)));

    let camera = Camera::new();

    let start = Instant::now();

    let buffer = ImageBuffer::from_fn(image_width, image_height, |i, j| {
        let mut color_sum = Color::zero();

        for _ in 0..samples {
            let u = (i as f64 + Random::f64()) / (image_width - 1) as f64;
            let v = ((image_height - 1 - j) as f64 + Random::f64()) / (image_height - 1) as f64;

            let ray = camera.get_ray(u, v);
            color_sum += ray_color(&ray, &world);
        }

        let sampled_color = color_sum / samples;
        Rgb(sampled_color.to_u8_range().into())
    });

    let end = Instant::now();

    buffer.save("sampled.png").expect("Failed saving image");
    let duration = end - start;
    println!("Rendered in {} ms", duration.as_millis());
}
