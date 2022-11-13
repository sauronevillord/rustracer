use rand::{self, Rng};

pub fn rand_f64_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>() * (max - min) + min
}

pub fn rand_f64() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

pub fn rand_bool() -> bool {
    let mut rng = rand::thread_rng();
    rng.gen::<bool>()
}