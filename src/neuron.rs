use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct Neuron {
    value: f64,
    bias: bool,
}

impl Neuron {
    pub(crate) fn new(bias: bool) -> Neuron {
        Neuron {
            value: if bias { 1.0 } else { 0.0 },
            bias,
        }
    }
}
