use std::sync::Arc;

use crate::{
    object::material::Material,
    util::Between,
    view::ray::{Hit, Ray, RayHit},
};

use super::vector::Vector3;

pub struct Sphere {
    center: Vector3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

// impl Default for Sphere {
//     fn default() -> Self {
//         Self {
//             center: Vector3::default(),
//             radius: 1.,
//         }
//     }
// }

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, range: (f64, f64)) -> Option<RayHit> {
        let origin_distance = *ray.origin() - self.center;
        let a = ray.direction().magnitude_squared();
        let half_b = Vector3::dot(ray.direction(), &origin_distance);
        let c = origin_distance.magnitude_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        let sqrt_discriminant = discriminant.sqrt();
        let mut root = (-half_b - sqrt_discriminant) / a;
        if !root.between(&range.0, &range.1) {
            root = (-half_b + sqrt_discriminant) / a;
            if !root.between(&range.0, &range.1) {
                return None;
            }
        }

        let hitpoint = ray.at(root);
        let normal = (hitpoint - self.center) / self.radius;
        let mut hit = RayHit {
            point: hitpoint,
            normal,
            front_face: false,
            t: root,
            material: self.material.clone(),
        };
        hit.set_face_normal(ray, normal);
        Some(hit)
    }
}
