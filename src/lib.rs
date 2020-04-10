#[cfg(test)]
#[macro_use]
extern crate approx;

pub mod genetic;
pub mod network;
pub mod neuron;
pub mod randomizer;
pub mod simulating_world;
pub mod simulation;
pub mod specimen;

const BIAS_VALUE: f64 = 1.0;
const MINIMUM_POPULATION_SIZE: usize = 4;
