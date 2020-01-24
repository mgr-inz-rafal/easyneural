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
    pub(crate) fixed_value: Option<f64>,
}

pub(crate) trait NeuronKind {
    fn get_value(&self) -> f64;
    fn set_value(&mut self, val: f64);
    fn get_fixed_value(&self) -> f64;
    fn set_fixed_value(&mut self, val: f64);
    fn get_inputs(&self) -> Option<&Vec<InputKind>>;
    fn get_inputs_mut(&mut self) -> Option<&mut Vec<InputKind>>;
    fn is_fixed_value(&self) -> bool;
}

impl NeuronKind for Neuron {
    fn is_fixed_value(&self) -> bool {
        self.fixed_value.is_some()
    }

    fn get_value(&self) -> f64 {
        if let Some(value) = self.value {
            return value;
        }
        panic!("Asking for a value of neuron without a value calculated");
    }

    fn get_fixed_value(&self) -> f64 {
        if let Some(value) = self.fixed_value {
            return value;
        }
        panic!("Asking for a fixed-value of neuron without a value calculated");
    }

    fn get_inputs(&self) -> Option<&Vec<InputKind>> {
        Some(&self.inputs)
    }

    fn get_inputs_mut(&mut self) -> Option<&mut Vec<InputKind>> {
        Some(&mut self.inputs)
    }

    fn set_value(&mut self, val: f64) {
        self.value = Some(val);
    }

    fn set_fixed_value(&mut self, val: f64) {
        self.fixed_value = Some(val);
    }
}

impl Neuron {
    // pub(crate) fn get_value(&self) -> f64 {
    //     if let Some(value) = self.fixed_value {
    //         return value;
    //     }
    //     self.value.unwrap()
    // }

    pub fn new() -> Neuron {
        Neuron {
            inputs: Vec::new(),
            value: None,
            fixed_value: None,
        }
    }

    pub(crate) fn fire(index: usize, neuron_repository: &mut Vec<Box<dyn NeuronKind>>) -> f64 {
        if neuron_repository[index].is_fixed_value() {
            return neuron_repository[index].get_fixed_value();
        }

        let mut sum = 0.0;

        // TODO: This solution with two separate loops is a dirty hack, rethink this
        if let Some(inputs) = neuron_repository[index].get_inputs_mut() {
            for input in inputs {
                match input {
                    InputKind::Value(cb) => {
                        let my_value = (cb.as_mut().unwrap())();
                        println!("\t\tValue: {}", my_value);
                        sum += my_value;
                    }
                    _ => {}
                }
            }
        }

        if let Some(inputs) = neuron_repository[index].get_inputs() {
            for input in inputs {
                match input {
                    InputKind::Axon(axon) => {
                        let my_weight = axon.get_weight();
                        let connecting_id = axon.get_id();
                        let connecting_value = neuron_repository[connecting_id].get_value();
                        println!(
                            "\t\tAxon: weight: {}, connecting_id: {}, connecting_value: {}",
                            my_weight, connecting_id, connecting_value
                        );
                        sum += my_weight * connecting_value;
                    }
                    _ => {}
                }
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
