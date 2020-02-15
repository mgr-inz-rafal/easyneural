pub(crate) struct NeuronValue {
    pub(crate) value: f64,
}

pub(crate) struct BiasNeuronValue;

pub(crate) trait Valued {
    fn get(&self) -> f64;
    fn set(&mut self, v: f64);
}

impl Valued for NeuronValue {
    fn get(&self) -> f64 {
        self.value
    }
    fn set(&mut self, v: f64) {
        self.value = v;
    }
}

impl Valued for BiasNeuronValue {
    fn get(&self) -> f64 {
        1.0
    }
    fn set(&mut self, _: f64) {}
}
