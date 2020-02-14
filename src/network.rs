use super::layer::{Layer, LayerBuilder};
use super::neuron::{InputKind, Neuron};
use super::neuron_repository::NeuronRepository;
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
    layers: Vec<Layer>,
    #[serde(skip_deserializing, skip_serializing)]
    toolbox: NetworkToolbox,
}

impl Network {
    fn new(layer_count: usize) -> Network {
        Network {
            layers: Vec::with_capacity(layer_count),
            toolbox: Default::default(),
        }
    }

    #[allow(dead_code)]
    fn setup_inputs(&mut self, inputs: Vec<fn() -> f64>, neuron_repository: &mut NeuronRepository) {
        inputs.iter().enumerate().for_each(|(index, input)| {
            let neuron_id = self.layers[0].neurons[index];
            let neuron = &mut neuron_repository.neurons[neuron_id];
            assert!(neuron.inputs.is_empty());
            neuron.inputs.push(InputKind::Value(Some(Box::new(*input))));
        });
    }

    fn create_layers(
        &mut self,
        neurons_in_layers: &[usize],
        bias: bool,
        neuron_repository: &mut NeuronRepository,
    ) {
        neurons_in_layers.iter().enumerate().for_each(|(index, _)| {
            self.layers.push(
                LayerBuilder::new()
                    .with_neuron_repository(&mut neuron_repository.neurons)
                    .with_neurons(neurons_in_layers[index])
                    .with_previous_layer(self.layers.last())
                    .with_bias(if index == neurons_in_layers.len() - 1 {
                        false
                    } else {
                        bias
                    })
                    .build(&mut self.toolbox.randomizer),
            );
        });
    }

    pub fn fire(&mut self, neuron_repository: &mut NeuronRepository) {
        self.layers.iter().for_each(|layer| {
            layer
                .neurons
                .iter()
                .for_each(|&neuron| neuron_repository.fire(neuron));
        });
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

    pub fn with_neurons_in_layers(mut self, neurons_in_layers: &[usize]) -> Self {
        self.neurons_in_layers.extend_from_slice(neurons_in_layers);
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

    pub fn build(mut self) -> (Network, NeuronRepository) {
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

        if self.bias {
            for i in 0..self.neurons_in_layers.len() - 1 {
                self.neurons_in_layers[i] += 1;
            }
        }

        let mut network = Network::new(self.neurons_in_layers.len());
        if let Some(custom_randomizer) = self.custom_randomizer {
            network.toolbox.randomizer = custom_randomizer;
        }

        let mut neuron_repository = NeuronRepository::new(self.neurons_in_layers.iter().sum());
        neuron_repository.neurons.push(Neuron::new());
        let neuron_buffer_address = &neuron_repository.neurons[0] as *const _;
        neuron_repository.neurons.clear();
        network.create_layers(&self.neurons_in_layers, self.bias, &mut neuron_repository);
        network.setup_inputs(
            self.inputs.as_ref().unwrap().to_vec(),
            &mut neuron_repository,
        );
        assert_eq!(
            &neuron_repository.neurons[0] as *const _, neuron_buffer_address,
            "Reallocation of the neuron buffer detected"
        );

        (network, neuron_repository)
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

        let (network, mut neuron_repository) = NetworkBuilder::new()
            .with_neurons_in_layers(&[3, 2, 5, 2])
            .with_inputs(vec![input1, input2, input3])
            .with_disabled_bias()
            .build();

        // Check number of layers
        assert_eq!(network.layers.len(), 4);

        // Check that inputs provide expected values
        let first_layer = &network.layers[0];
        let mut neuron_iterator = first_layer.neurons.iter();

        // TODO: Remove the copy&pasted boilerplate
        match &mut neuron_repository.neurons[*neuron_iterator.next().unwrap()].inputs[0] {
            InputKind::Value(cb) => assert!(relative_eq!(cb.as_mut().unwrap()(), 1.1)),
            _ => {}
        }

        match &mut neuron_repository.neurons[*neuron_iterator.next().unwrap()].inputs[0] {
            InputKind::Value(cb) => assert!(relative_eq!(cb.as_mut().unwrap()(), 2.2)),
            _ => {}
        }

        match &mut neuron_repository.neurons[*neuron_iterator.next().unwrap()].inputs[0] {
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
                    assert_eq!(neuron_repository.neurons[*neuron_id].inputs.len(), 1);
                }
            } else {
                // Validate that each neuron on the current layer
                // have exactly one axon per neuron in previous layer
                let neuron_count_on_previous_layer = network.layers[i - 1].neurons.len();
                for neuron_id in &network.layers[i].neurons {
                    assert_eq!(
                        neuron_repository.neurons[*neuron_id].inputs.len(),
                        neuron_count_on_previous_layer
                    );

                    // Validate that:
                    // - each axon really points to the neuron on previous layer
                    // - each axon points to different neuron
                    let mut processed_neurons = Vec::new();
                    for input in &neuron_repository.neurons[*neuron_id].inputs {
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

        let (network, mut network_repository) = NetworkBuilder::new()
            .with_neurons_in_layers(&[2, 2, 1])
            .with_inputs(vec![input1, input2])
            .with_custom_randomizer(custom_randomizer)
            .with_disabled_bias()
            .build();

        network_repository.neurons.iter_mut().for_each(|neuron| {
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
        let (network, mut neuron_repository) = NetworkBuilder::new()
            .with_neurons_in_layers(&[2, 2, 1])
            .with_inputs(vec![input1, input2])
            .with_custom_randomizer(custom_random_number_generator)
            .with_disabled_bias()
            .build();

        let mut index = 1;
        neuron_repository
            .neurons
            .iter_mut()
            .skip(2)
            .for_each(|neuron| {
                neuron.inputs.iter_mut().for_each(|input| {
                    match input {
                        InputKind::Value(cb) => {
                            let value = cb.as_mut().unwrap()();
                            assert!(relative_eq!(value, 1.1) || relative_eq!(value, 2.2))
                        }
                        InputKind::Axon(axon) => {
                            assert!(relative_eq!(axon.get_weight(), index as f64))
                        }
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
        let (mut network, mut neuron_repository) = NetworkBuilder::new()
            .with_neurons_in_layers(&[2, 2, 1])
            .with_inputs(vec![input1, input2])
            .with_custom_randomizer(custom_random_number_generator)
            .with_disabled_bias()
            .build();

        let serialized = serde_json::to_string(&network).unwrap();
        println!("{}", serialized); // TODO: Note to self - remove for release

        network.fire(&mut neuron_repository);

        let mut neuron = neuron_repository.neurons.iter_mut();
        assert!(relative_eq!(
            neuron.next().unwrap().value.as_ref().unwrap().get(),
            17.54
        ));
        assert!(relative_eq!(
            neuron.next().unwrap().value.as_ref().unwrap().get(),
            -9.214
        ));
        assert!(relative_eq!(
            neuron.next().unwrap().value.as_ref().unwrap().get(),
            1.0 * 17.54 + 2.0 * -9.214
        ));
        assert!(relative_eq!(
            neuron.next().unwrap().value.as_ref().unwrap().get(),
            3.0 * 17.54 + 4.0 * -9.214
        ));
        assert!(relative_eq!(
            neuron.next().unwrap().value.as_ref().unwrap().get(),
            (1.0 * 17.54 + 2.0 * -9.214) * 5.0 + (3.0 * 17.54 + 4.0 * -9.214) * 6.0
        ));

        let serialized = serde_json::to_string(&network).unwrap();
        println!("{}", serialized);
    }

    #[test]
    fn bias_neurons() {
        let input1 = || 1.1;
        let input2 = || 2.2;

        let neuron_count = [2, 3, 1];
        let (mut network, mut neuron_repository) = NetworkBuilder::new()
            .with_neurons_in_layers(&neuron_count)
            .with_inputs(vec![input1, input2])
            .build();

        // Validate that each layer but last has one more neuron
        let expected_neuron_count = [3, 4, 1];
        network
            .layers
            .iter()
            .map(|neuron_count| neuron_count.neurons.len())
            .zip(expected_neuron_count.iter())
            .for_each(|neuron_count| assert_eq!(neuron_count.0, *neuron_count.1));

        network.fire(&mut neuron_repository);

        // Validate that each bias neuron has the value of 1.0
        for i in 0..network.layers.len() - 1 {
            let last_neuron_index = network.layers[i].neurons.len() - 1;
            let neuron_id = network.layers[i].neurons[last_neuron_index];
            let neuron = &neuron_repository.neurons[neuron_id];
            assert!(relative_eq!(neuron.value.as_ref().unwrap().get(), 1.0));
        }

        let serialized = serde_json::to_string(&network).unwrap();
        println!("{}", serialized); // TODO: Note to self - remove for release
    }
}
