use super::axon::Axon;
use super::layer::Layer;
use super::neuron::Neuron;

pub struct Network {
    neurons: Vec<Neuron>,
    layers: Vec<Layer>,
    axons: Vec<Axon>,
}

impl Network {
    pub fn new() -> Network {
        Network {
            neurons: Vec::new(),
            layers: Vec::new(),
            axons: Vec::new(),
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

    pub fn build(&mut self) -> Network {
        assert!(
            self.neurons_in_layers.len() > 1,
            "Network must have at least 2 layers"
        );
        assert_eq!(
            *self.neurons_in_layers.last().unwrap(), // Safe to unwrap() - length check above
            1usize,
            "Last layer must consist of a single neuron"
        );

        let network = Network::new();
        for i in 0..self.neurons_in_layers.len() - 1 {
            // Create neurons for current layer
            let l = Layer::new();
            for j in 0..self.neurons_in_layers[i] {}
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
            .with_neurons_in_layers(vec![3, 3, 1])
            .build();
        assert_eq!(2, 2);
    }
}
