use super::axon_input::AxonInput;
use super::neuron::Neuron;
use id_arena::Id;

pub(crate) struct Axon {
    pub left: Id<Neuron>,
}

impl Axon {
    pub(crate) fn new(left: Id<Neuron>) -> Axon {
        Axon { left }
    }
}

impl AxonInput for Axon {
    fn get_value(&self) -> f64 {
        123.456
    }

    fn get_id(&self) -> Option<Id<Neuron>> {
        Some(self.left)
    }
}
