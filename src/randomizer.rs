use rand_distr::{Distribution, Normal};

pub trait RandomProvider {
    fn get_number(&mut self) -> f64;
}

pub struct DefaultRandomizer {
    sampler: Normal<f64>,
}

impl DefaultRandomizer {
    pub fn new() -> DefaultRandomizer {
        DefaultRandomizer {
            sampler: Normal::new(0.0, 1.0).expect("Unable to create randomizer"),
        }
    }
}

impl RandomProvider for DefaultRandomizer {
    fn get_number(&mut self) -> f64 {
        self.sampler.sample(&mut rand::thread_rng())
    }
}
