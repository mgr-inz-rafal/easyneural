use super::neuron::Neuron;
use id_arena::Id;

pub(crate) struct Axon {
    pub left: Id<Neuron>,
}

impl Axon {
    pub(crate) fn new(left: Id<Neuron>) -> Axon {
        Axon { left }
    }

    fn get_value() -> f64 {
        123.456
    }
}
