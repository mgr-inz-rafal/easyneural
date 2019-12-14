use super::neuron::Neuron;

pub struct Layer<'a> {
    previous: Option<Box<Layer<'a>>>,
    next: Option<Box<Layer<'a>>>,
    neurons: Vec<&'a Neuron>,
}

impl<'a> Layer<'a> {
    pub fn new() -> Layer<'a> {
        Layer {
            previous: None,
            next: None,
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
