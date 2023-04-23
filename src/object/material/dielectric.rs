use crate::{
    object::{geometry::vector::Vector3, material::color::Color},
    view::ray::{Ray, RayHit},
};

use super::{Material, Scatter};

pub struct Dielectric {
    /// Index of refraction
    index: f64,
}

impl Dielectric {
    pub fn new(index: f64) -> Self {
        Self { index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &RayHit) -> Option<Scatter> {
        let attenuation = Color::white();
        let refraction_ratio = if hit.front_face {
            1. / self.index
        } else {
            self.index
        };
        let normalized_ray_direction = ray.direction().normalize();
        let cos_theta = f64::min(Vector3::dot(&-normalized_ray_direction, &hit.normal), 1.);
        let sin_theta = (1. - cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.;
        let scattered_direction = if cannot_refract {
            normalized_ray_direction.reflect(&hit.normal)
        } else {
            normalized_ray_direction.refract(&hit.normal, refraction_ratio)
        };

        Some(Scatter {
            attenuation,
            ray: Ray::of(hit.point, scattered_direction),
        })
    }
}
