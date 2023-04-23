use crate::{
    object::geometry::vector::Vector3,
    view::ray::{Ray, RayHit},
};

use super::{color::Color, Material, Scatter};

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &RayHit) -> Option<Scatter> {
        let reflection_direction = ray.direction().reflect(&hit.normal).normalize();
        if Vector3::dot(&reflection_direction, &hit.normal) <= 0. {
            return None;
        }

        let reflection = Ray::of(
            hit.point,
            reflection_direction + self.fuzz * Vector3::random_in_unit_sphere(),
        );
        Some(Scatter {
            attenuation: self.albedo,
            ray: reflection,
        })
    }
}
