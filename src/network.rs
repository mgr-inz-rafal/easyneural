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
        for index in 0..self.neurons_in_layers.len() {
            // Create neurons for current layer
            let mut layer = Layer::new();
            for _ in 0..self.neurons_in_layers[index] {
                let mut new_neuron = Neuron::new();
                println!("Creating neuron on layer {}...", index);

                if index > 0 {
                    // Create axons connecting neurons on the current layer to
                    // all neurons on the previous layer
                    {
                        let previous_layer = &network.layers;
                        let previous_layer = previous_layer.last();
                        let previous_layer = previous_layer.as_ref().unwrap();

                        for n in &previous_layer.neurons {
                            let a = Axon::new(n);
                            new_neuron.inputs.push(a);
                        }
                    }
                }
                layer.add_neuron(new_neuron);
            }
            network.layers.push(layer);

            // // Create neurons for next layer
            // let mut ln = Layer::new();
            // for j in 0..self.neurons_in_layers[index + 1] {
            //     let mut n = Neuron::new();
            //     println!("Creating neuron on layer {}...", index + 1);

            //     // Create axons to each neuron of the previous layer
            //     for nn in &l.neurons {
            //         let a = Axon::new(nn);
            //         println!(
            //             "Creating axon from layer {} to layer {}...",
            //             index + 1,
            //             index
            //         );
            //         n.inputs.push(a); // TODO: Replace with NeuronBuilder
            //     }

            //     ln.add_neuron(n);
            //            }
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
        assert_eq!(2, 2);
    }
}
