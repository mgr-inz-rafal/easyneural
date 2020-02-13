#[cfg(test)]
#[macro_use]
extern crate approx;
#[macro_use]
extern crate if_chain;

pub mod axon;
pub mod layer;
pub mod network;
pub mod neuron;
pub mod neuron_repository;

use rand_distr::Normal;

thread_local! {
    pub static DEFAULT_RANDOM_SAMPLER: Normal<f64> = Normal::new(0.0, 1.0).unwrap();
}

#[cfg(test)]
mod tests {}
