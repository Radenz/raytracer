use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub},
};

use crate::util::random::Random;

type Values = [f64; 3];

#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    values: Values,
}

impl Vector3 {
    pub fn new(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Self {
        Self {
            values: [x.into(), y.into(), z.into()],
        }
    }

    pub fn zero() -> Self {
        Self::new(0, 0, 0)
    }

    pub fn ones() -> Self {
        Self::new(1, 1, 1)
    }

    pub fn dot(lhs: &Self, rhs: &Self) -> f64 {
        lhs[0] * rhs[0] + lhs[1] * rhs[1] + lhs[2] * rhs[2]
    }

    pub fn lerp(lhs: &Self, rhs: &Self, factor: f64) -> Self {
        (1. - factor) * *lhs + factor * *rhs
    }

    pub fn cross(lhs: &Self, rhs: &Self) -> Self {
        let mut values = [0.; 3];

        values[0] = lhs[1] * rhs[2] - lhs[2] * rhs[1];
        values[1] = lhs[2] * rhs[0] - lhs[0] * rhs[2];
        values[2] = lhs[0] * rhs[1] - lhs[1] * rhs[0];

        Self { values }
    }

    pub fn random() -> Self {
        Self::new(Random::f64(), Random::f64(), Random::f64())
    }

    pub fn random_between(min: f64, max: f64) -> Self {
        Self::new(
            Random::f64_between(min, max),
            Random::f64_between(min, max),
            Random::f64_between(min, max),
        )
    }

    pub fn random_inclusive_between(min: f64, max: f64) -> Self {
        Self::new(
            Random::f64_inclusive_between(min, max),
            Random::f64_inclusive_between(min, max),
            Random::f64_inclusive_between(min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let vector = Self::random_inclusive_between(-1., 1.);
            if vector.magnitude_squared() >= 1. {
                continue;
            };

            return vector;
        }
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let mut vector = Self::random_inclusive_between(-1., 1.);
            vector[2] = 0.;
            if vector.magnitude_squared() >= 1. {
                continue;
            };

            return vector;
        }
    }

    pub fn random_unit() -> Self {
        Self::random_in_unit_sphere().normalize()
    }

    pub fn random_in_hemisphere(normal: Self) -> Self {
        let random_in_unit_sphere = Self::random_in_unit_sphere();
        if Self::dot(&random_in_unit_sphere, &normal) > 0. {
            random_in_unit_sphere
        } else {
            -random_in_unit_sphere
        }
    }

    pub fn up() -> Self {
        Self {
            values: [0., 1., 0.],
        }
    }

    pub fn x(&self) -> f64 {
        self[0]
    }

    pub fn y(&self) -> f64 {
        self[1]
    }

    pub fn z(&self) -> f64 {
        self[2]
    }

    pub fn to_array(&self) -> [f64; 3] {
        [self.x(), self.y(), self.z()]
    }

    pub fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    pub fn magnitude_squared(&self) -> f64 {
        Self::dot(self, self)
    }

    pub fn normalize(&self) -> Self {
        *self / self.magnitude()
    }

    pub fn is_near_zero(&self) -> bool {
        let epsilon = 1e-8;
        self[0].abs() < epsilon && self[1].abs() < epsilon && self[2].abs() < epsilon
    }

    pub fn clamp_each(&self, min: impl Into<f64>, max: impl Into<f64>) -> Self {
        let min = min.into();
        let max = max.into();
        let mut vector = *self;
        for i in 0..3 {
            vector[i] = vector[i].clamp(min, max);
        }
        vector
    }

    pub fn sqrt(&self) -> Self {
        Self {
            values: [self.x().sqrt(), self.y().sqrt(), self.z().sqrt()],
        }
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        *self - 2. * Self::dot(self, normal) * *normal
    }

    pub fn refract(&self, normal: &Self, eta_ratio: f64) -> Self {
        let cos_theta = f64::min(Vector3::dot(&-*self, normal), 1.);
        let refraction_perpendicular = eta_ratio * (*self + cos_theta * *normal);
        let refraction_parallel =
            -(f64::abs(1. - refraction_perpendicular.magnitude_squared())).sqrt() * *normal;

        refraction_perpendicular + refraction_parallel
    }
}

unsafe impl Send for Vector3 {}

impl Sum for Vector3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vector3::zero(), |sum, el| sum + el)
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self[0], self[1], self[2])
    }
}

impl Default for Vector3 {
    fn default() -> Self {
        Self::zero()
    }
}

// ? Deref impls
// * If there is a better deref target for future
// * simplifications, impl Index & IndexMut instead

impl Deref for Vector3 {
    type Target = Values;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl DerefMut for Vector3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}

// ? Unary operators
impl Neg for Vector3 {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        for i in 0..3 {
            self[i] *= -1.;
        }
        self
    }
}

// ? Augmented operators
impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..3 {
            self[i] += rhs[i]
        }
    }
}

impl<T> MulAssign<T> for Vector3
where
    T: Into<f64>,
{
    fn mul_assign(&mut self, rhs: T) {
        let factor: f64 = rhs.into();

        for i in 0..3 {
            self[i] *= factor;
        }
    }
}

impl<T> DivAssign<T> for Vector3
where
    T: Into<f64>,
{
    fn div_assign(&mut self, rhs: T) {
        let factor: f64 = rhs.into();
        *self *= 1. / factor;
    }
}

// ? Binary operators
impl Add<Vector3> for Vector3 {
    type Output = Self;

    fn add(mut self, rhs: Vector3) -> Self::Output {
        for i in 0..3 {
            self[i] += rhs[i];
        }
        self
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Vector3) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(mut self, rhs: Vector3) -> Self::Output {
        for i in 0..3 {
            self[i] *= rhs[i];
        }
        self
    }
}

impl<T> Mul<T> for Vector3
where
    T: Into<f64>,
{
    type Output = Self;

    fn mul(mut self, rhs: T) -> Self::Output {
        let factor: f64 = rhs.into();
        for i in 0..3 {
            self[i] *= factor;
        }
        self
    }
}

macro_rules! impl_mul_vector3 {
    ($($t:ty),+) => {
        $(
            impl Mul<Vector3> for $t {
                type Output = Vector3;

                fn mul(self, rhs: Vector3) -> Self::Output {
                    rhs * self as f64
                }
            }
        )*
    };
}

impl_mul_vector3!(f64, f32, i64, i32, i16, i8, u64, u32, u16, u8);

impl<T> Div<T> for Vector3
where
    T: Into<f64>,
{
    type Output = Self;

    fn div(mut self, rhs: T) -> Self::Output {
        self /= rhs;
        self
    }
}

#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {
        Vector3::new($x, $y, $z)
    };
}
