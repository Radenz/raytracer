#![allow(unused)]

pub mod random;

use crate::{
    color::Color,
    vec::Vector3,
    view::ray::{Hit, HitTarget, Ray},
};

pub fn print_color(mut pixel_color: Color) {
    pixel_color *= 255.999;
    println!(
        "{} {} {}",
        pixel_color.x() as u8,
        pixel_color.y() as u8,
        pixel_color.z() as u8
    );
}

pub fn print_sampled_color(mut pixel_color: Color, samples: i32) {
    pixel_color /= samples;
    pixel_color = pixel_color.clamp_each(0, 0.999);
    pixel_color *= 256;
    println!(
        "{} {} {}",
        pixel_color.x() as u8,
        pixel_color.y() as u8,
        pixel_color.z() as u8
    );
}

fn hit_sphere(center: Vector3, radius: f64, ray: &Ray) -> f64 {
    let origin_distance = *ray.origin() - center;
    let a = ray.direction().magnitude_squared();
    let half_b = Vector3::dot(ray.direction(), &origin_distance);
    let c = origin_distance.magnitude_squared() - radius * radius;

    let discriminant = half_b * half_b - a * c;
    if discriminant < 0. {
        -1.
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

pub fn ray_color(ray: &Ray, world: &HitTarget) -> Color {
    if let Some(hit) = world.hit(&ray, (0., f64::INFINITY)) {
        return 0.5 * (hit.normal + Color::ones());
    }
    let normalized_direction = ray.direction().normalize();
    let t = 0.5 * (normalized_direction.y() + 1.);
    (1. - t) * Color::ones() + t * Color::new(0.5, 0.7, 1)
}

pub trait Between<T> {
    fn between(&self, lower_bound: &T, upper_bound: &T) -> bool;
}

impl<T> Between<T> for T
where
    T: PartialOrd,
{
    fn between(&self, lower_bound: &T, upper_bound: &T) -> bool {
        lower_bound <= self && self < upper_bound
    }
}
