use crate::view::ray::{Ray, RayHit};

use self::color::Color;

pub mod color;
pub mod dielectric;
pub mod lambertian;
pub mod metal;

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &RayHit) -> Option<Scatter>;
}

pub struct Scatter {
    pub attenuation: Color,
    pub ray: Ray,
}
