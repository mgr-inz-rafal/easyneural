use super::neuron::Neuron;

pub(crate) struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    pub(crate) fn new() -> Layer {
        Layer {
            neurons: Vec::new(),
        }
    }

    pub(crate) fn add_neuron(&mut self, neuron: Neuron) {
        self.neurons.push(neuron);
    }
}
