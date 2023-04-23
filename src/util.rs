#![allow(unused)]

pub mod random;

use crate::{
    object::{geometry::vector::Vector3, material::color::Color},
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

pub fn ray_color_diffuse(ray: &Ray, world: &HitTarget, depth: u32) -> Color {
    if depth == 0 {
        return Color::black();
    }

    if let Some(hit) = world.hit(&ray, (0.001, f64::INFINITY)) {
        if let Some(scatter) = hit.material.scatter(&ray, &hit) {
            return scatter.attenuation * ray_color_diffuse(&scatter.ray, &world, depth - 1);
        }
        return Color::black();
    }
    let normalized_direction = ray.direction().normalize();
    let t = 0.5 * (normalized_direction.y() + 1.);
    Vector3::lerp(&Color::ones(), &Color::new(0.5, 0.7, 1), t)
}

pub fn ray_color_diffuse_hemisphere(ray: &Ray, world: &HitTarget, depth: u32) -> Color {
    if depth == 0 {
        return Color::black();
    }

    if let Some(hit) = world.hit(&ray, (0.001, f64::INFINITY)) {
        let diffuse_target = hit.point + hit.normal + Vector3::random_in_hemisphere(hit.normal);
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
