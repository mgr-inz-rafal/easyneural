use super::axon::Axon;
use super::layer::Layer;
use serde::{Deserialize, Serialize};

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
        let mut sum = 0.0;

        // TODO: This solution with two separate loops is a dirty hack, rethink this
        for input in &mut neuron_repository[index].inputs {
            match input {
                InputKind::Value(cb) => {
                    let my_value = (cb.as_mut().unwrap())();
                    println!("\t\tValue: {}", my_value);
                    sum += my_value;
                }
                _ => {}
            }
        }

        for input in &neuron_repository[index].inputs {
            match input {
                InputKind::Axon(axon) => {
                    let my_weight = axon.get_weight();
                    let connecting_id = axon.get_id();
                    let connecting_value = &neuron_repository[connecting_id].value;
                    println!(
                        "\t\tAxon: weight: {}, connecting_id: {}, connecting_value: {}",
                        my_weight,
                        connecting_id,
                        connecting_value.unwrap()
                    );
                    sum += my_weight * connecting_value.unwrap();
                }
                _ => {}
            }
        }

        sum
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
