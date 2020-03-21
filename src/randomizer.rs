use rand_distr::{Distribution, Normal};

pub(crate) struct Randomizer {
    sampler: Normal<f64>,
}

impl Randomizer {
    pub(crate) fn new() -> Randomizer {
        Randomizer {
            sampler: Normal::new(0.0, 1.0).expect("Unable to create randomizer"),
        }
    }

    pub(crate) fn get_number(&self) -> f64 {
        self.sampler.sample(&mut rand::thread_rng())
    }
}
