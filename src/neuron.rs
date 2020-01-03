use super::axon_input::AxonInput;

pub(crate) struct Neuron {
    pub(crate) inputs: Vec<Box<dyn AxonInput>>,
}

impl Neuron {
    pub fn new() -> Neuron {
        Neuron { inputs: Vec::new() }
    }

    #[allow(dead_code)]
    pub(crate) fn set_input(&mut self, input: Box<dyn AxonInput>) {
        self.inputs.push(input);
    }
}
