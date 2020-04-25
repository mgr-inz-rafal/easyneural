use rand_distr::{Distribution, Normal};

/// A structure that can provide f64 values
///
/// `easyneural` comes with the default randomizer, but you can use
/// your favorite one by implementing this trait.
pub trait RandomProvider {
    /// Returns a number which will be treated as a next random number
    fn get_number(&mut self) -> f64;
}

/// Default randomizer
///
/// This randomizer is used in the nerual network if you
/// don't provide cusomized one.
pub struct DefaultRandomizer {
    sampler: Normal<f64>,
}

impl Default for DefaultRandomizer {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultRandomizer {
    pub fn new() -> DefaultRandomizer {
        DefaultRandomizer {
            sampler: Normal::new(0.0, 1.0).expect("Unable to create randomizer"),
        }
    }
}

impl RandomProvider for DefaultRandomizer {
    /// Returns next pseudo-random number from the OS
    fn get_number(&mut self) -> f64 {
        self.sampler.sample(&mut rand::thread_rng())
    }
}
