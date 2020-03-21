use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct Neuron {
    value: f64,
}

impl Neuron {
    pub(crate) fn new() -> Neuron {
        Neuron { value: 0.0 }
    }
}
