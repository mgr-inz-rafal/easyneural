use super::axon::Axon;
use super::layer::Layer;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Serialize, Deserialize)]
pub(crate) enum InputKind {
    Value(#[serde(skip)] Option<Box<dyn FnMut() -> f64>>),
    Axon(Axon),
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Neuron {
    pub(crate) inputs: Vec<InputKind>,
}

impl Neuron {
    pub fn new() -> Neuron {
        Neuron { inputs: Vec::new() }
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
                    neuron
                        .inputs
                        .push(InputKind::Axon(Axon::new(*n, (randomizer)())));
                });
            }
        }
        neuron
    }
}
