use super::axon::*;

pub struct Neuron<'a> {
    inputs: Vec<&'a Axon>,
    old_inputs: Vec<f64>,
}

impl<'a> Neuron<'a> {
    fn new() -> Neuron<'a> {
        Neuron {
            old_inputs: Vec::new(),
            inputs: Vec::new(),
        }
    }

    pub fn inputs(&self) -> &Vec<f64> {
        &self.old_inputs
    }

    pub fn add_axon(&mut self) {}
}

pub struct NeuronBuilder {
    neuron_count: usize,
    randomize_inputs: bool,
}

impl NeuronBuilder {
    pub fn new() -> NeuronBuilder {
        NeuronBuilder {
            neuron_count: 0,
            randomize_inputs: false,
        }
    }

    pub fn with_inputs(&mut self, count: usize) -> &mut Self {
        self.neuron_count = count;
        self
    }

    pub fn randomize_inputs(&mut self) -> &mut Self {
        self.randomize_inputs = true;
        self
    }

    pub fn build(&self) -> Neuron {
        let mut neuron = Neuron::new();
        for _ in 0..self.neuron_count {
            neuron.add_axon();
        }

        neuron
        /*
        use rand_distr::{Distribution, Normal};
        let mut n = Neuron::new();
        n.old_inputs.resize(self.neuron_count, 0.0);

        // TODO: Allow for different initialization methods (Xavier, He, etc.)
        if self.randomize_inputs {
            let normal = Normal::new(0.0, 1.0).unwrap();
            n.old_inputs
                .iter_mut()
                .for_each(|v| *v = normal.sample(&mut rand::thread_rng()));
        }
        n
        */
    }
}

/*
#[cfg(test)]
mod tests {
    use crate::neuron::*;
    #[test]
    fn create_neuron_with_inputs() {
        let n = NeuronBuilder::new().with_inputs(5).build();
        assert_eq!(5, n.inputs().len());
    }
}
*/
