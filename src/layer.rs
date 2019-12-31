use super::neuron::Neuron;
use id_arena::{Arena, Id};

pub(crate) struct Layer {
    pub(crate) neurons: Vec<Id<Neuron>>,
}

impl Layer {
    pub(crate) fn new() -> Layer {
        Layer {
            neurons: Vec::new(),
        }
    }
}
