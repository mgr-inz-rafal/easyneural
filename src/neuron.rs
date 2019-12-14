pub struct Neuron {
    pub inputs: Vec<f64>,
}

impl Neuron {
    fn new() -> Neuron {
        Neuron { inputs: Vec::new() }
    }
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
        use rand_distr::{Distribution, Normal};
        let mut n = Neuron::new();
        n.inputs.resize(self.neuron_count, 0.0);
        if self.randomize_inputs {
            let normal = Normal::new(0.0, 1.0).unwrap();
            n.inputs
                .iter_mut()
                .for_each(|v| *v = normal.sample(&mut rand::thread_rng()));
        }
        n
    }
}

#[cfg(test)]
mod tests {
    use crate::neuron::*;
    #[test]
    fn create_neuron_with_inputs() {
        let n = NeuronBuilder::new().with_inputs(5).build();
        assert_eq!(5, n.inputs.len());
    }
}
