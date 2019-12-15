use super::neuron::Neuron;

pub struct Layer<'a> {
    neurons: Vec<&'a Neuron>,
}

impl<'a> Layer<'a> {
    pub fn new() -> Layer<'a> {
        Layer {
            neurons: Vec::new(),
        }
    }

    pub fn add_neuron(&mut self, neuron: &'a Neuron) {
        self.neurons.push(neuron);
    }

    pub fn neurons(&self) -> &Vec<&'a Neuron> {
        &self.neurons
    }
}
