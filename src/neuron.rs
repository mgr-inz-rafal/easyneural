use crate::randomizer::RandomProvider;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct Neuron {
    value: Option<f64>,
    bias: bool,
    inputs: Vec<f64>,
}

impl Neuron {
    pub(crate) fn new(
        bias: bool,
        number_of_inputs: usize,
        randomizer: &mut Option<&mut dyn RandomProvider>,
    ) -> Neuron {
        Neuron {
            value: if bias { Some(1.0) } else { None },
            bias,
            inputs: {
                let mut inputs = Vec::with_capacity(number_of_inputs);
                if number_of_inputs > 0 {
                    let randomizer = randomizer.as_mut().expect("No randomizer provided");
                    for _ in 0..number_of_inputs {
                        inputs.push(randomizer.get_number());
                    }
                }
                inputs
            },
        }
    }
}
