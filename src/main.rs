#![allow(unused)]

use std::{
    rc::Rc,
    sync::Arc,
    time::{Duration, Instant},
};

use image::{save_buffer, ImageBuffer, Rgb};
use rayon::prelude::*;
use raytracer::{
    color::Color,
    geometry::sphere::Sphere,
    render::pixel::Vector3Extension,
    util::{print_color, print_sampled_color, random::Random, ray_color, ray_color_diffuse},
    vec::Vector3,
    vec3,
    view::{
        camera::{self, Camera},
        ray::{Hit, HitTarget, Ray},
    },
};

fn main() {
    gamma_corrected();
}

fn gamma_corrected() {
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

fn diffuse() {
    let aspect_ratio = 16. / 9.;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples = 100;
    let max_depth = 15;

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
                color_sum += ray_color(&ray, &world);
            }

            let mut sampled_color = color_sum / samples;
            *pixel = Rgb(sampled_color.to_u8_range().into());
        });

    let end = Instant::now();

    buffer.save("diffuse.png");
    let duration = end - start;
    println!("Rendered in {} ms", duration.as_millis());
}

fn sampled() {
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

        let mut sampled_color = color_sum / samples;

        Rgb(sampled_color.to_u8_range().into())
    });

    let end = Instant::now();

    buffer.save("world.png");
    let duration = end - start;
    println!("Rendered in {} ms", duration.as_millis());
}

fn world() {
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

fn sphere_normals2() {
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
            let sphere = Sphere::new(Vector3::new(0, 0, -1), 1.);

            let sphere_hit = sphere.hit(&ray, (0., f64::INFINITY));
            let pixel_color = match sphere_hit {
                None => {
                    let normalized_direction = ray.direction().normalize();
                    let factor = 0.5 * (normalized_direction.y() + 1.);
                    Vector3::lerp(&Color::new(1, 1, 1), &Color::new(0.5, 0.7, 1), factor)
                }
                Some(ray_hit) => ray_hit.normal + Vector3::new(1, 1, 1),
            };
            print_color(pixel_color);
        }
    }
}

fn sphere_normals() {
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
            // let pixel_color = ray_color(&ray);
            // print_color(pixel_color);
        }
    }
}

fn gradient() {
    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {j}");
        for i in 0..IMAGE_WIDTH {
            let red = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let green = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let blue = 0.25;

            let red_byte = (255.999 * red) as u8;
            let green_byte = (255.999 * green) as u8;
            let blue_byte = (255.999 * blue) as u8;

            println!("{} {} {}", red_byte, green_byte, blue_byte);
        }
    }
}
