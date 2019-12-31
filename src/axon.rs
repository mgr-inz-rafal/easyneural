use super::neuron::Neuron;
use id_arena::{Arena, Id};

pub(crate) struct Axon {
    left: Id<Neuron>,
}

impl Axon {
    pub(crate) fn new(left: Id<Neuron>) -> Axon {
        Axon { left }
    }
}
