use super::axon::Axon;
use super::axon_input::AxonInput;

pub(crate) struct Neuron {
    pub(crate) inputs: Vec<Box<dyn AxonInput>>,
}

impl Neuron {
    pub fn new() -> Neuron {
        Neuron { inputs: Vec::new() }
    }
}
