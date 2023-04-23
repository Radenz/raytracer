use crate::{
    object::geometry::vector::Vector3,
    view::ray::{Ray, RayHit},
};

use super::{color::Color, Material, Scatter};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit: &RayHit) -> Option<Scatter> {
        let mut scatter_direction = hit.normal + Vector3::random_unit();
        if scatter_direction.is_near_zero() {
            scatter_direction = hit.normal;
        }

        let scattered_ray = Ray::of(hit.point, scatter_direction);

        Some(Scatter {
            attenuation: self.albedo,
            ray: scattered_ray,
        })
    }
}
