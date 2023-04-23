use rand::{distributions::Uniform, prelude::Distribution, random};

pub struct Random;

impl Random {
    pub fn f64() -> f64 {
        random::<f64>()
    }

    pub fn f64_between(min: f64, max: f64) -> f64 {
        min + Self::f64() * (max - min)
    }

    pub fn f64_inclusive_between(min: f64, max: f64) -> f64 {
        let range = Uniform::from(min..=max);
        let mut rng = rand::thread_rng();
        range.sample(&mut rng)
    }
}
