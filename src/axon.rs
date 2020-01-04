use super::axon_input::AxonInput;
use super::neuron::Neuron;

pub(crate) struct Axon {
    pub left: usize,
}

impl Axon {
    pub(crate) fn new(left: usize) -> Axon {
        Axon { left }
    }
}

impl AxonInput for Axon {
    fn get_value(&self) -> f64 {
        123.456
    }

    fn get_id(&self) -> Option<usize> {
        Some(self.left)
    }
}
