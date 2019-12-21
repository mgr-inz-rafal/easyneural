use super::neuron::Neuron;

pub(crate) struct Axon<'a> {
    value: f64,
    left: &'a Neuron<'a>,
}

impl<'a> Axon<'a> {
    pub(crate) fn new(left: &'a Neuron) -> Axon<'a> {
        Axon { value: 0.0, left }
    }
}
