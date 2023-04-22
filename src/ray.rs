use crate::vec::Vector3;

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

pub trait Hit {
    fn hit(&self, ray: &Ray, range: (f64, f64)) -> Option<RayHit>;
}

pub struct RayHit {
    pub point: Vector3,
    pub normal: Vector3,
    pub t: f64,
}
