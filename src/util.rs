use crate::{color::Color, ray::Ray, vec::Vector3};

pub fn print_color(mut pixel_color: Color) {
    pixel_color *= 255.999;
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

pub fn ray_color(ray: &Ray) -> Color {
    let center = Vector3::new(0, 0, -1);
    let mut t = hit_sphere(center, 0.5, ray);

    if t > 0. {
        let hitpoint = ray.at(t);
        // Range -1 to 1 for each component
        let normal = (hitpoint - center).normalize();
        let ones = Vector3::new(1, 1, 1);
        return 0.5 * (normal + ones);
    }

    let normalized_direction = ray.direction().normalize();
    t = 0.5 * (normalized_direction.y() + 1.);
    (1. - t) * Color::new(1, 1, 1) + t * Color::new(0.5, 0.7, 1)
}
