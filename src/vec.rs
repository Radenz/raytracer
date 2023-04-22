use std::{
    fmt::Display,
    ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub},
};

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

    pub fn x(&self) -> f64 {
        self[0]
    }

    pub fn y(&self) -> f64 {
        self[1]
    }

    pub fn z(&self) -> f64 {
        self[2]
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