use super::neuron::Neuron;

pub(crate) struct Layer {
    pub(crate) neurons: Vec<usize>,
}

impl Layer {
    pub(crate) fn new() -> Layer {
        Layer {
            neurons: Vec::new(),
        }
    }
}
