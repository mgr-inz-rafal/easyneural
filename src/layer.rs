use super::neuron::Neuron;

pub(crate) struct Layer<'a> {
    pub(crate) neurons: Vec<Neuron<'a>>,
}

impl<'a> Layer<'a> {
    pub(crate) fn new() -> Layer<'a> {
        Layer {
            neurons: Vec::new(),
        }
    }

    pub(crate) fn add_neuron(&mut self, neuron: Neuron<'a>) {
        self.neurons.push(neuron);
    }
}
