use super::neuron::{Neuron, NeuronBuilder, Valued};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct Layer {
    pub(crate) neurons: Vec<usize>,
}

impl Layer {
    pub(crate) fn new() -> Layer {
        Layer {
            neurons: Vec::new(),
        }
    }
}

pub(crate) struct LayerBuilder<'a> {
    number_of_neurons: Option<usize>,
    neuron_repository: Option<&'a mut Vec<Box<dyn Valued>>>,
    previous_layer: Option<&'a Layer>,
    bias: bool,
}

impl<'a> LayerBuilder<'a> {
    pub fn new() -> LayerBuilder<'a> {
        LayerBuilder {
            number_of_neurons: None,
            neuron_repository: None,
            previous_layer: None,
            bias: false,
        }
    }

    pub fn with_bias(mut self, bias: bool) -> Self {
        self.bias = bias;
        self
    }

    pub fn with_neurons(mut self, numbner_of_neurons: usize) -> Self {
        self.number_of_neurons = Some(numbner_of_neurons);
        self
    }

    pub fn with_neuron_repository(
        mut self,
        neuron_repository: &'a mut Vec<Box<dyn Valued>>,
    ) -> Self {
        self.neuron_repository = Some(neuron_repository);
        self
    }

    pub fn with_previous_layer(mut self, layer: Option<&'a Layer>) -> Self {
        self.previous_layer = layer;
        self
    }

    #[allow(clippy::borrowed_box)]
    pub fn build(&mut self, mut randomizer: &mut Box<(dyn FnMut() -> f64 + 'static)>) -> Layer {
        let mut layer = Layer::new();
        let previous_layer = self.previous_layer;
        if_chain! {
            if let Some(number_of_neurons) = self.number_of_neurons;
            if let Some(ref mut neuron_repository) = self.neuron_repository;
            then {
                let mut new_neuron_id = None;
                (0..number_of_neurons).for_each(|_| {
                    neuron_repository.push(
                        Box::new(NeuronBuilder::new()
                            .with_connection_to_layer(previous_layer)
                            .build(&mut randomizer)),
                    );
                    new_neuron_id = Some(neuron_repository.len() - 1);
                    layer.neurons.push(new_neuron_id.unwrap());
                });
                if self.bias {
                    neuron_repository[new_neuron_id.unwrap()].set_fixed_value(1.0);
                }
            }
        }
        layer
    }
}
