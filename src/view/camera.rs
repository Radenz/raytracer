use crate::object::geometry::vector::Vector3;

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
    // View vectors
    front: Vector3,
    right: Vector3,
    up: Vector3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        position: Vector3,
        at: Vector3,
        mut up: Vector3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let half_height = (theta / 2.).tan();
        let viewport_height = 2. * half_height;
        let viewport_width = aspect_ratio * viewport_height;

        let front = (position - at).normalize();
        let right = Vector3::cross(&up, &front).normalize();
        up = Vector3::cross(&front, &right);

        let horizontal = focus_distance * viewport_width * right;
        let vertical = focus_distance * viewport_height * up;
        let lower_left_corner =
            position - (horizontal / 2) - (vertical / 2) - focus_distance * front;
        let lens_radius = aperture / 2.;

        Self {
            origin: position,
            lower_left_corner,
            horizontal,
            vertical,
            front,
            right,
            up,
            lens_radius,
        }
    }

    pub fn get_ray(&self, u: impl Into<f64>, v: impl Into<f64>) -> Ray {
        let u: f64 = u.into();
        let v: f64 = v.into();

        let random_displacement = self.lens_radius * Vector3::random_in_unit_disk();
        let offset = self.right * random_displacement.x() + self.up * random_displacement.y();

        Ray::of(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        )
    }
}
