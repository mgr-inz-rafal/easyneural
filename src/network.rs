use super::axon::Axon;
use super::layer::Layer;

pub struct Network<'a> {
    layers: Vec<Layer<'a>>,
    axons: Vec<Axon>,
}

impl<'a> Network<'a> {
    pub fn new() -> Network<'a> {
        Network {
            layers: Vec::new(),
            axons: Vec::new(),
        }
    }

    pub fn add_layer(&mut self, layer: Layer<'a>) {
        self.layers.push(layer);
    }

    pub fn layers(&self) -> &Vec<Layer<'a>> {
        &self.layers
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
