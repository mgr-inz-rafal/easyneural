#[cfg(test)]
#[macro_use]
extern crate approx;

use serde::Deserialize;

mod genetic;
pub(crate) mod network;
mod neuron;

/// Randomizer implementation.
pub mod randomizer;

/// World used for training the network.
pub mod simulating_world;

/// Lerning routines.
pub mod simulation;

/// Interfacing with `easyneural`.
pub mod specimen;

/// Training ground for testing the trained network.
pub mod training_ground;

const BIAS_VALUE: f64 = 1.0;
const MINIMUM_POPULATION_SIZE: usize = 4;

/// Specimen is used to exchange the neural network data
/// with the outside world.
///
/// This is the struct you use for transferring the
/// neural network instances to and from the `easyneural` crate.
#[derive(Clone, Debug, Deserialize)]
pub struct Specimen {
    pub brain: network::NetworkLayout,
    pub fitness: f64,
}

impl Specimen {
    pub fn from_json(j: &str) -> Self {
        Specimen {
            fitness: 0.0,
            brain: serde_json::from_str(j).unwrap(), // TODO: Add error checking
        }
    }
}
