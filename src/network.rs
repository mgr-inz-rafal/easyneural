use super::axon_input::AxonInput;
use super::layer::{Layer, LayerBuilder};
use super::neuron::Neuron;
use rand_distr::Distribution;
use serde::{Deserialize, Serialize, Serializer};

fn value_closure_serialize<S: Serializer>(
    f: &Option<fn() -> f64>,
    s: S,
) -> Result<S::Ok, S::Error> {
    s.serialize_f64(f.unwrap()())
}

#[derive(Serialize, Deserialize)]
struct NetworkInput {
    #[serde(skip_deserializing, serialize_with = "value_closure_serialize")]
    value_provider: Option<fn() -> f64>,
    weight: f64,
}

impl NetworkInput {
    #[allow(dead_code)]
    fn new(value_provider: fn() -> f64, weight: f64) -> NetworkInput {
        NetworkInput {
            value_provider: Some(value_provider),
            weight,
        }
    }
}

#[typetag::serde]
impl AxonInput for NetworkInput {
    fn get_value(&self) -> f64 {
        if let Some(value_provider) = self.value_provider {
            (value_provider)()
        } else {
            panic!("Empty value provider");
        }
    }

    fn get_id(&self) -> Option<usize> {
        None
    }

    fn get_weight(&self) -> f64 {
        self.weight
    }
}

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
            neuron.set_input(Box::new(NetworkInput::new(
                *input,
                (self.toolbox.randomizer)(),
            )));
        });
    }

    fn create_layers(&mut self, neurons_in_layers: &Vec<usize>) {
        neurons_in_layers.iter().enumerate().for_each(|(index, _)| {
            self.layers.push(
                LayerBuilder::new()
                    .with_neuron_repository(&mut self.neurons)
                    .with_neurons(neurons_in_layers[index])
                    .with_previous_layer(self.layers.last())
                    .build(&mut self.toolbox.randomizer),
            );
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
}

impl NetworkBuilder {
    pub fn new() -> NetworkBuilder {
        NetworkBuilder {
            neurons_in_layers: Vec::new(),
            inputs: None,
            custom_randomizer: None,
        }
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
        network.create_layers(&self.neurons_in_layers);
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

        let network = NetworkBuilder::new()
            .with_neurons_in_layers(vec![3, 2, 5, 2])
            .with_inputs(vec![input1, input2, input3])
            .build();

        // Check number of layers
        assert_eq!(network.layers.len(), 4);

        // Check that inputs provide expected values
        let first_layer = &network.layers[0];
        let mut neuron_iterator = first_layer.neurons.iter();
        assert!(relative_eq!(
            network.neurons[*neuron_iterator.next().unwrap()].inputs[0].get_value(),
            1.1
        ));
        assert!(relative_eq!(
            network.neurons[*neuron_iterator.next().unwrap()].inputs[0].get_value(),
            2.2
        ));
        assert!(relative_eq!(
            network.neurons[*neuron_iterator.next().unwrap()].inputs[0].get_value(),
            3.3
        ));

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

                    // TODO:
                    // Reconsider the check below. It requires the trait to expose
                    // the get_id() function, which is not used in any other place.
                    //
                    //
                    // Validate that:
                    // - each axon really points to the neuron on previous layer
                    // - each axon points to different neuron
                    let mut processed_neurons = Vec::new();
                    for axon in &network.neurons[*neuron_id].inputs {
                        if let Some(ref neuron_id) = &axon.get_id() {
                            assert!(!processed_neurons.contains(neuron_id));
                            assert_eq!(
                                network.layers[i - 1]
                                    .neurons
                                    .iter()
                                    .filter(|x| *x == neuron_id)
                                    .count(),
                                1
                            );
                            processed_neurons.push(*neuron_id);
                        } else {
                            panic!("Found AxonInput-object without a neuron id. Is this the first layer of the network?");
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

        let network = NetworkBuilder::new()
            .with_neurons_in_layers(vec![2, 2, 1])
            .with_inputs(vec![input1, input2])
            .with_custom_randomizer(custom_randomizer)
            .build();

        network.neurons.iter().for_each(|neuron| {
            neuron
                .inputs
                .iter()
                .for_each(|input| assert!(relative_eq!(input.get_weight(), 17.2)))
        });
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
        let network = NetworkBuilder::new()
            .with_neurons_in_layers(vec![2, 2, 1])
            .with_inputs(vec![input1, input2])
            .with_custom_randomizer(custom_random_number_generator)
            .build();

        let mut index = 1;
        network.neurons.iter().skip(2).for_each(|neuron| {
            neuron.inputs.iter().for_each(|input| {
                assert!(relative_eq!(input.get_weight(), index as f64));
                index += 1;
            })
        });
    }
}
