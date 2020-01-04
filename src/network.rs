use super::axon::Axon;
use super::axon_input::AxonInput;
use super::layer::Layer;
use super::neuron::Neuron;
use serde::{Deserialize, Serialize, Serializer};

fn value_closure_serialize<S>(foo: &Option<fn() -> f64>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_f64(foo.unwrap()())
}

#[derive(Serialize, Deserialize)]
struct NetworkInput {
    #[serde(skip_deserializing, serialize_with = "value_closure_serialize")]
    value_provider: Option<fn() -> f64>,
}

impl NetworkInput {
    #[allow(dead_code)]
    fn new(value_provider: fn() -> f64) -> NetworkInput {
        NetworkInput {
            value_provider: Some(value_provider),
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
        16.6
    }
}

#[derive(Serialize, Deserialize)]
pub struct Network {
    neurons: Vec<Neuron>,
    layers: Vec<Layer>,
}

impl Network {
    fn new() -> Network {
        Network {
            neurons: Vec::new(),
            layers: Vec::new(),
        }
    }

    #[allow(dead_code)]
    fn setup_inputs(&mut self, inputs: Vec<fn() -> f64>) {
        inputs.iter().enumerate().for_each(|(index, input)| {
            let neuron_id = self.layers[0].neurons[index];
            let neuron = &mut self.neurons[neuron_id];
            assert!(neuron.inputs.is_empty());
            neuron.set_input(Box::new(NetworkInput::new(*input)));
        });
    }
}

pub struct NetworkBuilder {
    neurons_in_layers: Vec<usize>,
    inputs: Option<Vec<fn() -> f64>>,
}

impl NetworkBuilder {
    pub fn new() -> NetworkBuilder {
        NetworkBuilder {
            neurons_in_layers: Vec::new(),
            inputs: None,
        }
    }

    pub fn with_neurons_in_layers(&mut self, neurons_in_layers: Vec<usize>) -> &mut Self {
        self.neurons_in_layers = neurons_in_layers;
        self
    }

    pub fn with_inputs(&mut self, inputs: Vec<fn() -> f64>) -> &mut Self {
        self.inputs = Some(inputs);
        self
    }

    fn connect_neuron_to_layer(
        &self,
        new_neuron: usize,
        layer: Option<&Layer>,
        neurons: &mut Vec<Neuron>,
    ) {
        if let Some(last_layer) = layer {
            last_layer.neurons.iter().for_each(|n| {
                neurons[new_neuron].inputs.push(Box::new(Axon::new(*n)));
            })
        } else {
            panic!("Trying to connect a neuron to the non-existing layer");
        }
    }

    fn create_layer(&self, network: &mut Network, i: usize) -> Layer {
        let mut new_layer = Layer::new();
        (0..self.neurons_in_layers[i]).for_each(|_| {
            network.neurons.push(Neuron::new());
            let new_neuron = network.neurons.len() - 1;
            if i > 0 {
                self.connect_neuron_to_layer(
                    new_neuron,
                    network.layers.last(),
                    &mut network.neurons,
                );
            }
            new_layer.neurons.push(new_neuron);
        });
        new_layer
    }

    pub fn build(&self) -> Network {
        assert!(
            self.neurons_in_layers.len() > 1,
            "Network must have at least 2 layers"
        );
        assert_eq!(
            *self.neurons_in_layers.last().unwrap(), // Safe to unwrap() - length check above
            1usize,
            "Last layer must consist of a single neuron"
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

        let mut network = Network::new();
        self.neurons_in_layers
            .iter()
            .enumerate()
            .for_each(|(i, _)| {
                let new_layer = self.create_layer(&mut network, i);
                network.layers.push(new_layer);
            });
        network.setup_inputs(self.inputs.as_ref().unwrap().to_vec());
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
            .with_neurons_in_layers(vec![3, 2, 2, 1])
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
        assert_eq!(layer_iterator.next().unwrap().neurons.len(), 2);
        assert_eq!(layer_iterator.next().unwrap().neurons.len(), 1);

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
}
