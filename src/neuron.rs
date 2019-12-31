use super::axon::Axon;

pub(crate) struct Neuron {
    pub(crate) inputs: Vec<Axon>,
}

impl Neuron {
    pub fn new() -> Neuron {
        Neuron { inputs: Vec::new() }
    }
}
