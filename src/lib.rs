#[cfg(test)]
#[macro_use]
extern crate approx;

pub mod network;
pub mod neuron;
pub mod randomizer;
pub mod trainer;

const BIAS_VALUE: f64 = 1.0;
