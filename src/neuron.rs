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
    pub(crate) value: Option<f64>,
}

impl Neuron {
    pub fn new() -> Neuron {
        Neuron {
            inputs: Vec::new(),
            value: None,
        }
    }

    pub(crate) fn fire(index: usize, neuron_repository: &mut Vec<Neuron>) -> f64 {
        let sum = 0.0;
        for input in &mut neuron_repository[index].inputs {
            match input {
                InputKind::Axon(axon) => {
                    let my_weight = axon.get_weight();
                    println!("\t\tAxon: weight: {}, connecting_value: {}", my_weight, 0.0);
                }
                InputKind::Value(ref mut cb) => {
                    let my_value = (cb.as_mut().unwrap())();
                    println!("\t\tValue: {}", my_value);
                }
            }
        }

        777777.7
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
