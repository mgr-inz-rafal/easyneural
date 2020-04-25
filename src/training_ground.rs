use crate::network::{Network, NetworkBuilder};
use crate::randomizer::DefaultRandomizer;

pub struct Exercise {
    specimen: crate::Specimen,
}

impl Exercise {
    pub fn new(specimen: &crate::Specimen) -> Self {
        Exercise {
            specimen: specimen.clone(),
        }
    }

    pub fn get_output(&self, inputs: &Vec<f64>) -> f64 {
        let neurons_per_layer = [1];
        let mut randomizer = DefaultRandomizer::new();
        let mut net = NetworkBuilder::new()
            .with_neurons_per_layer(&neurons_per_layer)
            .with_randomizer(&mut randomizer)
            .build();

        net.layout = self.specimen.brain.clone();

        net.fire(&inputs);
        net.get_output()[0]
    }
}
