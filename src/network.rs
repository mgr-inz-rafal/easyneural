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
        for layer_index in 0..self.neurons_in_layers.len() {
            let mut new_layer = Layer::new();
            for _ in 0..self.neurons_in_layers[layer_index] {
                let new_neuron = network.neurons.alloc(Neuron::new());
                println!("Creating neuron in layer {}", layer_index);

                if layer_index > 0 {
                    let last_layer = network.layers.last();
                    if let Some(last_layer) = last_layer {
                        for previous_neuron_id in &last_layer.neurons {
                            println!(
                                "\tCreating axon between layer {} and {}",
                                layer_index,
                                layer_index - 1
                            );
                            network.neurons[new_neuron]
                                .inputs
                                .push(Axon::new(*previous_neuron_id));
                        }
                    }
                }
                new_layer.neurons.push(new_neuron);
            }
            network.layers.push(new_layer);
        }
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
    }
}
