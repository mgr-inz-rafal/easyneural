use crate::neuron::Neuron;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Network {
    neurons: Vec<Neuron>,
    layers: Vec<Vec<usize>>,
}

impl Network {
    fn new(layer_count: usize) -> Network {
        Network {
            neurons: Vec::new(),
            layers: {
                let mut layers = Vec::new();
                layers.resize(layer_count, Vec::new());
                layers
            },
        }
    }
}

pub struct NetworkBuilder<'a> {
    neurons_per_layer: Option<&'a [usize]>,
}

impl<'a> NetworkBuilder<'a> {
    pub fn new() -> NetworkBuilder<'a> {
        NetworkBuilder {
            neurons_per_layer: None,
        }
    }

    pub fn with_neurons_per_layer(&mut self, neurons_per_layer: &'a [usize]) -> &mut Self {
        self.neurons_per_layer = Some(neurons_per_layer);
        self
    }

    pub fn build(&self) -> Network {
        if let Some(neurons_per_layer) = self.neurons_per_layer {
            let mut net = Network::new(neurons_per_layer.len());
            for layer_index in 0..neurons_per_layer.len() {
                for _ in 0..neurons_per_layer[layer_index] {
                    net.neurons.push(Neuron::new(false));
                    net.layers[layer_index].push(net.neurons.len() - 1);
                }
                if layer_index != neurons_per_layer.len() - 1 {
                    net.layers[layer_index].push(net.neurons.len() - 1);
                    net.neurons.push(Neuron::new(true));
                }
            }
            net
        } else {
            panic!("Neurons per layer not set");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::network::*;
    #[test]
    fn network_structure() {
        let neurons_per_layer = [2, 3, 1];
        let net = NetworkBuilder::new()
            .with_neurons_per_layer(&neurons_per_layer)
            .build();

        let mut expected_neurons: Vec<usize> = neurons_per_layer.iter().map(|x| x + 1).collect();
        if let Some(last) = expected_neurons.last_mut() {
            *last -= 1;
        }

        assert_eq!(net.layers.len(), neurons_per_layer.len());

        net.layers
            .iter()
            .zip(expected_neurons.iter())
            .for_each(|(x, y)| assert_eq!(x.len(), *y));

        let serialized = serde_json::to_string(&net).unwrap();
        println!("{}", serialized);
    }
}
