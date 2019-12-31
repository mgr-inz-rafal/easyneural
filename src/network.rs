use super::axon::Axon;
use super::layer::Layer;
use super::neuron::Neuron;
use id_arena::{Arena, Id};

pub struct Network {
    neurons: Arena<Neuron>,
    layers: Vec<Layer>,
}

impl Network {
    pub fn new() -> Network {
        Network {
            neurons: Arena::new(),
            layers: Vec::new(),
        }
    }
}

pub struct NetworkBuilder {
    neurons_in_layers: Vec<usize>,
}

impl NetworkBuilder {
    pub fn new() -> NetworkBuilder {
        NetworkBuilder {
            neurons_in_layers: Vec::new(),
        }
    }

    pub fn with_neurons_in_layers(&mut self, neurons_in_layers: Vec<usize>) -> &mut Self {
        self.neurons_in_layers = neurons_in_layers;
        self
    }

    fn connect_neuron_to_layer(
        &self,
        new_neuron: &Id<Neuron>,
        layer: Option<&Layer>,
        neurons: &mut Arena<Neuron>,
    ) {
        if let Some(last_layer) = layer {
            last_layer.neurons.iter().for_each(|n| {
                let a = Axon::new(*n);
                neurons[*new_neuron].inputs.push(a);
            })
        } else {
            panic!("Trying to connect a neuron to the non-existing layer");
        }
    }

    fn create_layer(&self, network: &mut Network, i: usize) -> Layer {
        let mut new_layer = Layer::new();
        (0..self.neurons_in_layers[i]).for_each(|_| {
            let new_neuron = network.neurons.alloc(Neuron::new());
            if i > 0 {
                self.connect_neuron_to_layer(
                    &new_neuron,
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

        let mut network = Network::new();
        self.neurons_in_layers
            .iter()
            .enumerate()
            .for_each(|(i, _)| {
                let new_layer = self.create_layer(&mut network, i);
                network.layers.push(new_layer);
            });
        network
    }
}

#[cfg(test)]
mod tests {
    use crate::network::*;
    #[test]
    fn build_network() {
        let nb = NetworkBuilder::new()
            .with_neurons_in_layers(vec![3, 2, 2, 1])
            .build();

        // Check number of layers
        assert_eq!(nb.layers.len(), 4);

        // Check number of neurons per layer
        let mut layer_iterator = nb.layers.iter();
        assert_eq!(layer_iterator.next().unwrap().neurons.len(), 3);
        assert_eq!(layer_iterator.next().unwrap().neurons.len(), 2);
        assert_eq!(layer_iterator.next().unwrap().neurons.len(), 2);
        assert_eq!(layer_iterator.next().unwrap().neurons.len(), 1);

        // Validate proper connections between neurons
        nb.layers.iter().enumerate().for_each(|(i, _)| {
            if i == 0 {
                // Neurons on the first layer should have no input
                for neuron_id in &nb.layers[i].neurons {
                    assert!(nb.neurons[*neuron_id].inputs.is_empty());
                }
            } else {
                // Validate that each neuron on the current layer
                // have exactly one axon per neuron in previous layer
                let neuron_count_on_previous_layer = nb.layers[i - 1].neurons.len();
                for neuron_id in &nb.layers[i].neurons {
                    assert_eq!(
                        nb.neurons[*neuron_id].inputs.len(),
                        neuron_count_on_previous_layer
                    );

                    // Validate that:
                    // - each axon really points to the neuron on previous layer
                    // - each axon points to different neuron
                    let mut processed_neurons = Vec::new();
                    for axon in &nb.neurons[*neuron_id].inputs {
                        assert!(!processed_neurons.contains(&axon.left));
                        assert_eq!(
                            nb.layers[i - 1]
                                .neurons
                                .iter()
                                .filter(|x| **x == axon.left)
                                .count(),
                            1
                        );
                        processed_neurons.push(axon.left);
                    }
                }
            }
        });
    }
}
