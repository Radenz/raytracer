#![allow(unused)]

use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use image::{ImageBuffer, Rgb};
use rayon::prelude::*;
use raytracer::{
    object::{
        geometry::{sphere::Sphere, vector::Vector3},
        material::{color::Color, lambertian::Lambertian, metal::Metal},
    },
    render::pixel::Vector3Extension,
    util::{random::Random, ray_color_diffuse},
    vec3,
    view::{camera::Camera, ray::HitTarget},
};

// ? This lambertian surface approximation use normal displacement for diffuse target
// direction
fn main() {
    let aspect_ratio = 16. / 9.;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples = 100;
    let max_depth = 50;

    let mut world = HitTarget::new();

    let ground = Lambertian::new(vec3![0.8, 0.8, 0]);
    let pink_clay = Lambertian::new(vec3![0.7, 0.3, 0.3]);
    let gray_metal = Metal::new(vec3![0.8, 0.8, 0.8]);
    let gold_metal = Metal::new(vec3![0.8, 0.6, 0.2]);

    (*world).push(Arc::new(Sphere::new(
        vec3![0, 0, -1],
        0.5,
        Arc::new(pink_clay),
    )));
    (*world).push(Arc::new(Sphere::new(
        vec3![-1, 0, -1],
        0.5,
        Arc::new(gray_metal),
    )));
    (*world).push(Arc::new(Sphere::new(
        vec3![1, 0, -1],
        0.5,
        Arc::new(gold_metal),
    )));
    (*world).push(Arc::new(Sphere::new(
        vec3![0, -100.5, -1],
        100.,
        Arc::new(ground),
    )));

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

    buffer.save("metal-material.png");
    let duration = end - start;
    println!("Rendered in {} ms", duration.as_millis());
}
