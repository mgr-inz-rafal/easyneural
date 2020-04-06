use crate::randomizer::RandomProvider;
use crate::BIAS_VALUE;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Neuron {
    pub(crate) value: Option<f64>,
    bias: bool,
    pub(crate) inputs: Vec<f64>,
}

impl Neuron {
    pub(crate) fn new(
        bias: bool,
        number_of_inputs: usize,
        randomizer: &mut Option<&mut dyn RandomProvider>,
    ) -> Neuron {
        Neuron {
            value: if bias { Some(BIAS_VALUE) } else { None },
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
