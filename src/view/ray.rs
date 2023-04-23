use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use crate::object::geometry::vector::Vector3;

pub struct Ray {
    origin: Vector3,
    direction: Vector3,
}

impl Ray {
    pub fn new() -> Self {
        Self {
            origin: Vector3::zero(),
            direction: Vector3::zero(),
        }
    }

    pub fn of(origin: Vector3, direction: Vector3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> &Vector3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vector3 {
        &self.direction
    }

    pub fn at(&self, t: impl Into<f64>) -> Vector3 {
        let t: f64 = t.into();
        self.origin + t * self.direction
    }
}

pub trait Hit: Sync + Send {
    fn hit(&self, ray: &Ray, range: (f64, f64)) -> Option<RayHit>;
}

pub struct RayHit {
    pub point: Vector3,
    pub normal: Vector3,
    pub t: f64,
    pub front_face: bool,
}

impl RayHit {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3) {
        self.front_face = Vector3::dot(&ray.direction, &outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub struct HitTarget {
    targets: Vec<Arc<dyn Hit>>,
}

impl HitTarget {
    pub fn new() -> Self {
        Self { targets: vec![] }
    }
}

impl Hit for HitTarget {
    fn hit(&self, ray: &Ray, mut range: (f64, f64)) -> Option<RayHit> {
        let mut ray_hit: Option<RayHit> = None;
        let mut closest_hit_distance = range.1;

        for object in self.targets.iter() {
            range = (range.0, closest_hit_distance);
            if let Some(local_hit) = object.hit(ray, range) {
                closest_hit_distance = local_hit.t;
                ray_hit = Some(local_hit);
            }
        }

        ray_hit
    }
}

impl Deref for HitTarget {
    type Target = Vec<Arc<dyn Hit>>;

    fn deref(&self) -> &Self::Target {
        &self.targets
    }
}

impl DerefMut for HitTarget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.targets
    }
}
