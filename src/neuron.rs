use super::axon::Axon;
use super::layer::Layer;
use super::network::NeuronRepository;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;

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

pub(crate) struct BiasNeuron {}

impl NeuronKind for BiasNeuron {
    fn get_value(&self) -> f64 {
        1.0
    }

    fn get_inputs(&self) -> Option<&Vec<InputKind>> {
        None
    }

    fn get_inputs_mut(&mut self) -> Option<&mut Vec<InputKind>> {
        None
    }

    fn set_value(&mut self, _: f64) {
        panic!("Cannot set value of the bias neuron")
    }

    fn fire(&mut self, neuron_repository: &Rc<RefCell<NeuronRepository>>) -> Option<f64> {
        None
    } // Bias neurons do not fire
}

pub(crate) trait NeuronKind {
    fn get_value(&self) -> f64;
    fn set_value(&mut self, val: f64);
    fn get_inputs(&self) -> Option<&Vec<InputKind>>;
    fn get_inputs_mut(&mut self) -> Option<&mut Vec<InputKind>>;
    fn fire(&mut self, neuron_repository: &Rc<RefCell<NeuronRepository>>) -> Option<f64>;
}

impl NeuronKind for Neuron {
    fn get_value(&self) -> f64 {
        if let Some(value) = self.value {
            return value;
        }
        panic!("Asking for a value of neuron without a value calculated");
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

    fn fire(&mut self, neuron_repository: &Rc<RefCell<NeuronRepository>>) -> Option<f64> {
        let mut sum = 0.0;

        // TODO: This solution with two separate loops is a dirty hack, rethink this
        //        if let Some(inputs) = (*neuron_repository).borrow_mut()[index].get_inputs_mut() {
        for input in &mut self.inputs {
            match input {
                InputKind::Value(cb) => {
                    let my_value = (cb.as_mut().unwrap())();
                    println!("\t\tValue: {}", my_value);
                    sum += my_value;
                }
                _ => {}
            }
        }
        //      }

        //if let Some(inputs) = neuron_repository[index].get_inputs() {
        for input in &self.inputs {
            match input {
                InputKind::Axon(axon) => {
                    let my_weight = axon.get_weight();
                    let connecting_id = axon.get_id();
                    let connecting_value =
                        (*(*neuron_repository).borrow()).neurons[connecting_id].get_value();
                    println!(
                        "\t\tAxon: weight: {}, connecting_id: {}, connecting_value: {}",
                        my_weight, connecting_id, connecting_value
                    );
                    sum += my_weight * connecting_value;
                }
                _ => {}
            }
        }
        //}

        Some(sum)
    }
}

impl BiasNeuron {
    pub fn new() -> BiasNeuron {
        BiasNeuron {}
    }
}

impl Neuron {
    pub fn new() -> Neuron {
        Neuron {
            inputs: Vec::new(),
            value: None,
        }
    }
}

pub(crate) struct NeuronBuilder<'a> {
    layer: Option<&'a Layer>,
    bias: Option<bool>,
}

impl<'a> NeuronBuilder<'a> {
    pub fn new() -> NeuronBuilder<'a> {
        NeuronBuilder {
            layer: None,
            bias: None,
        }
    }

    pub fn with_connection_to_layer(mut self, layer: Option<&'a Layer>) -> Self {
        self.layer = layer;
        self
    }

    pub fn make_bias(mut self) -> Self {
        self.bias = Some(true);
        self
    }

    fn is_bias(&self) -> bool {
        match self.bias {
            Some(bias) => bias,
            None => false,
        }
    }

    #[allow(clippy::borrowed_box)]
    pub fn build(
        self,
        randomizer: &mut Box<(dyn FnMut() -> f64 + 'static)>,
    ) -> Box<dyn NeuronKind> {
        if self.is_bias() {
            let mut neuron = BiasNeuron::new();
            return Box::new(neuron);
        } else {
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
            return Box::new(neuron);
        }
    }
}
