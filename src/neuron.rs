use super::axon::Axon;

pub struct Neuron {
    inputs: Vec<Axon>,
    output: Option<f64>,
}

impl Neuron {
    pub fn new() -> Neuron {
        Neuron {
            inputs: Vec::new(),
            output: None,
        }
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
