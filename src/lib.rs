#[cfg(test)]
#[macro_use]
extern crate approx;

pub mod axon;
pub mod axon_input;
pub mod layer;
pub mod network;
pub mod neuron;

use rand_distr::Normal;

thread_local! {
    pub static DEFAULT_RANDOM_SAMPLER: Normal<f64> = Normal::new(0.0, 1.0).unwrap();
}

#[cfg(test)]
mod tests {}
