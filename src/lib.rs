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

struct NeuronValue {
    value: f64,
}

struct BiasNeuronValue;

trait Valued {
    fn get(&self) -> f64;
    fn set(&mut self, v: f64);
}

impl Valued for NeuronValue {
    fn get(&self) -> f64 {
        self.value
    }
    fn set(&mut self, v: f64) {
        self.value = v;
    }
}

impl Valued for BiasNeuronValue {
    fn get(&self) -> f64 {
        1.0
    }
    fn set(&mut self, _: f64) {}
}

#[cfg(test)]
mod tests {}
