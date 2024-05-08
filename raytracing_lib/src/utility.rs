use rand::prelude::*;

pub fn random_double() -> f64 {
    random::<f64>()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    thread_rng().gen_range(min..max)
}
