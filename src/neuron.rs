use super::axon::Axon;

pub(crate) struct Neuron<'a> {
    pub(crate) inputs: Vec<Axon<'a>>,
    output: Option<f64>,
}

impl<'a> Neuron<'a> {
    pub fn new() -> Neuron<'a> {
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
