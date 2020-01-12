use super::axon::Axon;
use super::axon_input::AxonInput;
use super::layer::Layer;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) enum InputKind {
    Value(f64),
    Neuron(usize),
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Neuron {
    pub(crate) inputs: Vec<Box<dyn AxonInput>>,
    pub(crate) inputs_1: Vec<InputKind>,
}

impl Neuron {
    pub fn new() -> Neuron {
        Neuron {
            inputs: Vec::new(),
            inputs_1: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn set_input(&mut self, input: Box<dyn AxonInput>) {
        self.inputs.push(input);
    }
}

pub(crate) struct NeuronBuilder<'a> {
    layer: Option<&'a Layer>,
}

impl<'a> NeuronBuilder<'a> {
    pub fn new() -> NeuronBuilder<'a> {
        NeuronBuilder { layer: None }
    }

    pub fn with_connection_to_layer(mut self, layer: Option<&'a Layer>) -> Self {
        self.layer = layer;
        self
    }

    #[allow(clippy::borrowed_box)]
    pub fn build(self, randomizer: &mut Box<(dyn FnMut() -> f64 + 'static)>) -> Neuron {
        let mut neuron = Neuron::new();
        if let Some(layer) = self.layer.as_ref() {
            {
                layer.neurons.iter().for_each(|n| {
                    neuron.inputs.push(Box::new(Axon::new(*n, (randomizer)())));
                });
            }
        }
        neuron
    }
}
