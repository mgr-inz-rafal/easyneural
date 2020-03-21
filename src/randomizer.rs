use rand_distr::{Distribution, Normal};

pub trait RandomProvider {
    fn get_number(&mut self) -> f64;
}

pub(crate) struct Randomizer {
    sampler: Normal<f64>,
}

pub(crate) struct FixedRandomizer {
    current: f64,
}

impl FixedRandomizer {
    pub(crate) fn new() -> FixedRandomizer {
        FixedRandomizer { current: 0.0 }
    }
}

impl Randomizer {
    pub(crate) fn new() -> Randomizer {
        Randomizer {
            sampler: Normal::new(0.0, 1.0).expect("Unable to create randomizer"),
        }
    }
}

impl RandomProvider for Randomizer {
    fn get_number(&mut self) -> f64 {
        self.sampler.sample(&mut rand::thread_rng())
    }
}

impl RandomProvider for FixedRandomizer {
    fn get_number(&mut self) -> f64 {
        self.current += 1.5;
        self.current
    }
}
