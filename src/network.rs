use super::axon::Axon;
use super::layer::Layer;
use super::neuron::Neuron;

pub struct Network<'a> {
    layers: Vec<Layer<'a>>,
    axons: Vec<Axon<'a>>,
}

impl<'a> Network<'a> {
    pub fn new() -> Network<'a> {
        Network {
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
        let mut index = 1;
        loop {
            // Create neurons for current layer
            let mut l = Layer::new();
            for j in 0..self.neurons_in_layers[index] {
                l.add_neuron(Neuron::new());
            }

            // Create neurons for next layer
            let mut ln = Layer::new();
            for j in 0..self.neurons_in_layers[index + 1] {
                let mut n = Neuron::new();

                // Create axons to each neuron of the previous layer
                for nn in &l.neurons {
                    let a = Axon::new(nn);
                    n.inputs.push(a); // TODO: Replace with NeuronBuilder
                }

                ln.add_neuron(n);
            }
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
