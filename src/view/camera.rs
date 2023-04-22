use crate::{vec::Vector3, vec3};

use super::ray::Ray;

pub struct Camera {
    // For perspective projection
    origin: Vector3,
    // Bottom left corner of projection plane
    lower_left_corner: Vector3,
    // Horizontal displacement
    horizontal: Vector3,
    // Vertical displacement
    vertical: Vector3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16. / 9.;
        let viewport_height = 2.;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.;

        let origin = Vector3::zero();
        let horizontal = vec3![viewport_width, 0, 0];
        let vertical = vec3![viewport_width, 0, 0];
        let lower_left_corner = origin - horizontal / 2 - vertical / 2 - vec3![0, 0, focal_length];

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: impl Into<f64>, v: impl Into<f64>) -> Ray {
        let u: f64 = u.into();
        let v: f64 = v.into();

        Ray::of(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical,
        )
    }
}
