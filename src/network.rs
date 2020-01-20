use super::layer::{Layer, LayerBuilder};
use super::neuron::{InputKind, Neuron};
use rand_distr::Distribution;
use serde::{Deserialize, Serialize};

struct NetworkToolbox {
    randomizer: Box<dyn FnMut() -> f64>,
}

impl Default for NetworkToolbox {
    fn default() -> Self {
        NetworkToolbox {
            randomizer: Box::new(default_randomizer),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Network {
    neurons: Vec<Neuron>,
    layers: Vec<Layer>,
    #[serde(skip_deserializing, skip_serializing)]
    toolbox: NetworkToolbox,
}

impl Network {
    fn new(layer_count: usize, neuron_count: usize) -> Network {
        Network {
            neurons: Vec::with_capacity(neuron_count),
            layers: Vec::with_capacity(layer_count),
            toolbox: Default::default(),
        }
    }

    #[allow(dead_code)]
    fn setup_inputs(&mut self, inputs: Vec<fn() -> f64>) {
        inputs.iter().enumerate().for_each(|(index, input)| {
            let neuron_id = self.layers[0].neurons[index];
            let neuron = &mut self.neurons[neuron_id];
            assert!(neuron.inputs.is_empty());
            neuron.inputs.push(InputKind::Value(Some(Box::new(*input))));
        });
    }

    fn create_layers(&mut self, neurons_in_layers: &[usize], bias: bool) {
        neurons_in_layers.iter().enumerate().for_each(|(index, _)| {
            self.layers.push(
                LayerBuilder::new()
                    .with_neuron_repository(&mut self.neurons)
                    .with_neurons(neurons_in_layers[index])
                    .with_previous_layer(self.layers.last())
                    .with_bias(bias)
                    .build(&mut self.toolbox.randomizer),
            );
        });
    }

    pub fn fire(&mut self) {
        for layer_id in 0..self.layers.len() {
            println!("Firing layer {}", layer_id);
            for neuron_index in 0..self.layers[layer_id].neurons.len() {
                let neuron_id = self.layers[layer_id].neurons[neuron_index];
                let new_value = Neuron::fire(neuron_id, &mut self.neurons);
                self.set_neuron_value(neuron_id, new_value);
            }
        }
    }

    fn set_neuron_value(&mut self, neuron_index: usize, value: f64) {
        println!(
            "\t\t\tSetting neuron at index {} to {}",
            neuron_index, value
        );
        self.neurons[neuron_index].value = Some(value);
    }
}

fn default_randomizer() -> f64 {
    crate::DEFAULT_RANDOM_SAMPLER.with(|sampler| sampler.sample(&mut rand::thread_rng()))
}

pub struct NetworkBuilder {
    neurons_in_layers: Vec<usize>,
    inputs: Option<Vec<fn() -> f64>>,
    custom_randomizer: Option<Box<dyn FnMut() -> f64>>,
    bias: bool,
}

impl NetworkBuilder {
    pub fn new() -> NetworkBuilder {
        NetworkBuilder {
            neurons_in_layers: Vec::new(),
            inputs: None,
            custom_randomizer: None,
            bias: true,
        }
    }

    pub fn with_disabled_bias(mut self) -> Self {
        self.bias = false;
        self
    }

    pub fn with_neurons_in_layers(mut self, neurons_in_layers: Vec<usize>) -> Self {
        self.neurons_in_layers = neurons_in_layers;
        self
    }

    pub fn with_inputs(mut self, inputs: Vec<fn() -> f64>) -> Self {
        self.inputs = Some(inputs);
        self
    }

    pub fn with_custom_randomizer(mut self, f: impl FnMut() -> f64 + 'static) -> Self {
        self.custom_randomizer = Some(Box::new(f));
        self
    }

    pub fn build(self) -> Network {
        assert!(
            self.neurons_in_layers.len() > 1,
            "Network must have at least 2 layers"
        );
        assert!(
            self.inputs.as_ref().is_some(),
            "No network inputs provided, use with_inputs() function"
        );
        assert_eq!(
            self.neurons_in_layers[0],
            self.inputs.as_ref().unwrap().len(),
            "Number of neurons on the first layer must be the same as number of inputs"
        );

        let mut network = Network::new(
            self.neurons_in_layers.len(),
            self.neurons_in_layers.iter().sum(),
        );

        if let Some(custom_randomizer) = self.custom_randomizer {
            network.toolbox.randomizer = custom_randomizer;
        }

        network.neurons.push(Neuron::new());
        let neuron_buffer_address = &network.neurons[0] as *const _;
        network.neurons.clear();
        network.create_layers(&self.neurons_in_layers, self.bias);
        network.setup_inputs(self.inputs.as_ref().unwrap().to_vec());
        assert_eq!(
            &network.neurons[0] as *const _, neuron_buffer_address,
            "Reallocation of the neuron buffer detected"
        );

        network
    }
}

#[cfg(test)]
mod tests {
    use crate::network::*;
    #[test]
    fn network_structure() {
        let input1 = || 1.1;
        let input2 = || 2.2;
        let input3 = || 3.3;

        let mut network = NetworkBuilder::new()
            .with_neurons_in_layers(vec![3, 2, 5, 2])
            .with_inputs(vec![input1, input2, input3])
            .with_disabled_bias()
            .build();

        // Check number of layers
        assert_eq!(network.layers.len(), 4);

        // Check that inputs provide expected values
        let first_layer = &network.layers[0];
        let mut neuron_iterator = first_layer.neurons.iter();

        // TODO: Remove the copy&pasted boilerplate
        match &mut network.neurons[*neuron_iterator.next().unwrap()].inputs[0] {
            InputKind::Value(cb) => assert!(relative_eq!(cb.as_mut().unwrap()(), 1.1)),
            _ => {}
        }

        match &mut network.neurons[*neuron_iterator.next().unwrap()].inputs[0] {
            InputKind::Value(cb) => assert!(relative_eq!(cb.as_mut().unwrap()(), 2.2)),
            _ => {}
        }

        match &mut network.neurons[*neuron_iterator.next().unwrap()].inputs[0] {
            InputKind::Value(cb) => assert!(relative_eq!(cb.as_mut().unwrap()(), 3.3)),
            _ => {}
        }

        // Check number of neurons per layer
        let mut layer_iterator = network.layers.iter();
        assert_eq!(layer_iterator.next().unwrap().neurons.len(), 3);
        assert_eq!(layer_iterator.next().unwrap().neurons.len(), 2);
        assert_eq!(layer_iterator.next().unwrap().neurons.len(), 5);
        assert_eq!(layer_iterator.next().unwrap().neurons.len(), 2);

        // Validate proper connections between neurons
        network.layers.iter().enumerate().for_each(|(i, _)| {
            if i == 0 {
                // Neurons on the first layer should have exactly one input
                for neuron_id in &network.layers[i].neurons {
                    assert_eq!(network.neurons[*neuron_id].inputs.len(), 1);
                }
            } else {
                // Validate that each neuron on the current layer
                // have exactly one axon per neuron in previous layer
                let neuron_count_on_previous_layer = network.layers[i - 1].neurons.len();
                for neuron_id in &network.layers[i].neurons {
                    assert_eq!(
                        network.neurons[*neuron_id].inputs.len(),
                        neuron_count_on_previous_layer
                    );

                    // Validate that:
                    // - each axon really points to the neuron on previous layer
                    // - each axon points to different neuron
                    let mut processed_neurons = Vec::new();
                    for input in &network.neurons[*neuron_id].inputs {
                        if let InputKind::Axon(axon) = input {
                            assert!(!processed_neurons.contains(&axon.get_id()));
                            assert_eq!(
                                network.layers[i - 1]
                                    .neurons
                                    .iter()
                                    .filter(|x| *x == &axon.get_id())
                                    .count(),
                                1
                            );
                            processed_neurons.push(*neuron_id);
                        }
                    }
                }
            }
        });
    }

    #[test]
    fn custom_randomizer_as_funtion() {
        let input1 = || 1.1;
        let input2 = || 2.2;

        fn custom_randomizer() -> f64 {
            17.2
        }

        let mut network = NetworkBuilder::new()
            .with_neurons_in_layers(vec![2, 2, 1])
            .with_inputs(vec![input1, input2])
            .with_custom_randomizer(custom_randomizer)
            .with_disabled_bias()
            .build();

        network.neurons.iter_mut().for_each(|neuron| {
            neuron.inputs.iter_mut().for_each(|input| {
                match input {
                    InputKind::Value(cb) => {
                        let value = cb.as_mut().unwrap()();
                        assert!(relative_eq!(value, 1.1) || relative_eq!(value, 2.2))
                    }
                    InputKind::Axon(axon) => assert!(relative_eq!(axon.get_weight(), 17.2)),
                };
            })
        })
    }

    #[test]
    fn custom_randomizer_as_capturing_closure() {
        let input1 = || 1.1;
        let input2 = || 2.2;

        let mut current_random_value = 0.0;
        let custom_random_number_generator = move || {
            current_random_value += 1.0;
            current_random_value
        };
        let mut network = NetworkBuilder::new()
            .with_neurons_in_layers(vec![2, 2, 1])
            .with_inputs(vec![input1, input2])
            .with_custom_randomizer(custom_random_number_generator)
            .with_disabled_bias()
            .build();

        let mut index = 1;
        network.neurons.iter_mut().skip(2).for_each(|neuron| {
            neuron.inputs.iter_mut().for_each(|input| {
                match input {
                    InputKind::Value(cb) => {
                        let value = cb.as_mut().unwrap()();
                        assert!(relative_eq!(value, 1.1) || relative_eq!(value, 2.2))
                    }
                    InputKind::Axon(axon) => assert!(relative_eq!(axon.get_weight(), index as f64)),
                };
                index += 1;
            })
        });
    }

    #[test]
    fn network_in_action() {
        let input1 = || 17.54;
        let input2 = || -9.214;

        let mut current_random_value = 0.0;
        let custom_random_number_generator = move || {
            current_random_value += 1.0;
            current_random_value
        };
        let mut network = NetworkBuilder::new()
            .with_neurons_in_layers(vec![2, 2, 1])
            .with_inputs(vec![input1, input2])
            .with_custom_randomizer(custom_random_number_generator)
            .with_disabled_bias()
            .build();

        let serialized = serde_json::to_string(&network).unwrap();
        println!("{}", serialized); // TODO: Note to self - remove for release

        network.fire();

        let mut neuron = network.neurons.iter_mut();
        assert!(relative_eq!(neuron.next().unwrap().value.unwrap(), 17.54));
        assert!(relative_eq!(neuron.next().unwrap().value.unwrap(), -9.214));
        assert!(relative_eq!(
            neuron.next().unwrap().value.unwrap(),
            1.0 * 17.54 + 2.0 * -9.214
        ));
        assert!(relative_eq!(
            neuron.next().unwrap().value.unwrap(),
            3.0 * 17.54 + 4.0 * -9.214
        ));
        assert!(relative_eq!(
            neuron.next().unwrap().value.unwrap(),
            (1.0 * 17.54 + 2.0 * -9.214) * 5.0 + (3.0 * 17.54 + 4.0 * -9.214) * 6.0
        ));

        let serialized = serde_json::to_string(&network).unwrap();
        println!("{}", serialized);
    }

    #[test]
    fn bias_neurons() {
        let input1 = || 1.1;
        let input2 = || 2.2;

        fn custom_randomizer() -> f64 {
            100.0
        }

        let mut network = NetworkBuilder::new()
            .with_neurons_in_layers(vec![2, 2, 1])
            .with_inputs(vec![input1, input2])
            .with_custom_randomizer(custom_randomizer)
            .build();

        network.neurons.iter_mut().for_each(|neuron| {
            neuron.inputs.iter_mut().for_each(|input| {
                match input {
                    InputKind::Value(cb) => {
                        let value = cb.as_mut().unwrap()();
                        assert!(relative_eq!(value, 1.1) || relative_eq!(value, 2.2))
                    }
                    InputKind::Axon(axon) => assert!(relative_eq!(axon.get_weight(), 100.0)),
                };
            })
        })
    }
}
