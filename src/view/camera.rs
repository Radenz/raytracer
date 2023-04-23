use crate::{object::geometry::vector::Vector3, vec3};

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
    pub fn new(
        position: Vector3,
        at: Vector3,
        mut up: Vector3,
        vfov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let half_height = (theta / 2.).tan();
        let viewport_height = 2. * half_height;
        let viewport_width = aspect_ratio * viewport_height;

        let front = (position - at).normalize();
        let right = Vector3::cross(&up, &front).normalize();
        up = Vector3::cross(&front, &right);

        let horizontal = viewport_width * right;
        let vertical = viewport_height * up;
        let lower_left_corner = position - (horizontal / 2) - (vertical / 2) - front;

        Self {
            origin: position,
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
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
