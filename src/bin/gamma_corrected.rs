#![allow(unused)]

use std::{
    rc::Rc,
    sync::Arc,
    time::{Duration, Instant},
};

use image::{save_buffer, ImageBuffer, Rgb};
use rayon::prelude::*;
use raytracer::{
    object::{
        geometry::{sphere::Sphere, vector::Vector3},
        material::color::Color,
    },
    render::pixel::Vector3Extension,
    util::random::Random,
    vec3,
    view::{
        camera::Camera,
        ray::{Hit, HitTarget, Ray},
    },
};

fn main() {
    let aspect_ratio = 16. / 9.;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples = 100;
    let max_depth = 50;

    let mut world = HitTarget::new();
    (*world).push(Arc::new(Sphere::new(vec3![0, 0, -1], 0.5)));
    (*world).push(Arc::new(Sphere::new(vec3![0, -100.5, -1], 100.)));

    let camera = Camera::new();

    let start = Instant::now();

    let mut buffer = ImageBuffer::new(image_width, image_height);
    buffer
        .enumerate_pixels_mut()
        .par_bridge()
        .into_par_iter()
        .for_each(|(i, mut j, pixel)| {
            let mut color_sum = Vector3::zero();
            for _ in 0..samples {
                let u = (i as f64 + Random::f64()) / (image_width - 1) as f64;
                let v = ((image_height - 1 - j) as f64 + Random::f64()) / (image_height - 1) as f64;

                let ray = camera.get_ray(u, v);
                color_sum += ray_color_diffuse(&ray, &world, max_depth);
            }

            let mut sampled_color = color_sum / samples;

            *pixel = Rgb(sampled_color.sqrt().to_u8_range().into());
        });

    let end = Instant::now();

    buffer.save("gamma-corrected.png");
    let duration = end - start;
    println!("Rendered in {} ms", duration.as_millis());
}

fn ray_color_diffuse(ray: &Ray, world: &HitTarget, depth: u32) -> Color {
    if depth == 0 {
        return Color::black();
    }

    if let Some(hit) = world.hit(&ray, (0., f64::INFINITY)) {
        let diffuse_target = hit.point + hit.normal + Vector3::random_unit();
        return 0.5
            * ray_color_diffuse(
                &Ray::of(hit.point, diffuse_target - hit.point),
                world,
                depth - 1,
            );
    }
    let normalized_direction = ray.direction().normalize();
    let t = 0.5 * (normalized_direction.y() + 1.);
    Vector3::lerp(&Color::ones(), &Color::new(0.5, 0.7, 1), t)
}
