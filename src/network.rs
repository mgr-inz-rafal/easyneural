use crate::neuron::Neuron;
use crate::randomizer::{DefaultRandomizer, FixedRandomizer, RandomProvider};
use if_chain::if_chain;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct NetworkLayout {
    neurons: Vec<Neuron>,
    layers: Vec<Vec<usize>>,
}

pub struct Network {
    layout: NetworkLayout,
    activator: fn(f64) -> f64,
}

impl Network {
    fn new(neurons_per_layers: &[usize], activator: fn(f64) -> f64) -> Network {
        Network {
            layout: NetworkLayout {
                neurons: Vec::with_capacity(
                    neurons_per_layers.iter().sum::<usize>() + neurons_per_layers.len() - 1,
                ),
                layers: {
                    let mut layers = Vec::new();
                    layers.resize(neurons_per_layers.len(), Vec::new());
                    layers
                },
            },
            activator,
        }
    }

    #[allow(dead_code)]
    fn set_layer_values(layer: &mut Vec<usize>, input_values: &[f64], neurons: &mut Vec<Neuron>) {
        layer
            .iter()
            .zip(input_values.iter())
            .for_each(|(neuron_id, input_value)| {
                neurons[*neuron_id].value = Some(*input_value);
            });
    }

    #[allow(dead_code)]
    fn fire_layer(
        layer: &Vec<usize>,
        prev_layer: &Vec<usize>,
        neurons: &mut Vec<Neuron>,
        is_last: bool,
        activator: fn(f64) -> f64,
    ) {
        for i in 0..layer.len() - if is_last { 0 } else { 1 } {
            let mut value = 0.0;
            let neuron_index = layer[i];
            for j in 0..prev_layer.len() {
                let input_index = j;
                let input_value = neurons[neuron_index].inputs[input_index];
                let prev_layer_neuron_index = prev_layer[j];
                let prev_layer_neuron_value = neurons[prev_layer_neuron_index]
                    .value
                    .expect("Neuron w/o value found");
                value += input_value * prev_layer_neuron_value;
            }
            neurons[neuron_index].value = Some(activator(value));
        }
    }

    #[allow(dead_code)]
    fn fire(&mut self, input_values: &[f64]) {
        assert!(
            self.layout.layers.len() > 0,
            "Trying to fire network with no layers"
        );
        assert_eq!(
            input_values.len(),
            self.layout.layers[0].len() - 1,
            "Incorrent number of inputs"
        );

        Network::set_layer_values(
            &mut self.layout.layers[0],
            input_values,
            &mut self.layout.neurons,
        );
        for layer_index in 1..self.layout.layers.len() {
            Network::fire_layer(
                &self.layout.layers[layer_index],
                &self.layout.layers[layer_index - 1],
                &mut self.layout.neurons,
                if layer_index == self.layout.layers.len() - 1 {
                    true
                } else {
                    false
                },
                self.activator,
            );
        }
    }
}

pub struct NetworkBuilder<'a> {
    neurons_per_layer: Option<&'a [usize]>,
    randomizer: Option<&'a mut dyn RandomProvider>,
    activator: Option<fn(f64) -> f64>,
}

impl<'a> NetworkBuilder<'a> {
    pub fn new() -> NetworkBuilder<'a> {
        NetworkBuilder {
            neurons_per_layer: None,
            randomizer: None,
            activator: None,
        }
    }

    pub fn with_neurons_per_layer(&mut self, neurons_per_layer: &'a [usize]) -> &mut Self {
        self.neurons_per_layer = Some(neurons_per_layer);
        self
    }

    pub fn with_randomizer(&mut self, randomizer: &'a mut dyn RandomProvider) -> &mut Self {
        self.randomizer = Some(randomizer);
        self
    }

    pub fn with_activator(&mut self, activator: fn(f64) -> f64) -> &mut Self {
        self.activator = Some(activator);
        self
    }

    fn number_of_neurons_on_previous_layer(
        &self,
        layer_index: usize,
        neurons_per_layer: &[usize],
    ) -> usize {
        if layer_index == 0 {
            0
        } else {
            neurons_per_layer[layer_index - 1] + 1
        }
    }

    pub fn build(&mut self) -> Network {
        if_chain! {
            if let Some(neurons_per_layer) = self.neurons_per_layer;
            if let Some(activator) = self.activator;
            then
            {
                let mut net = Network::new(neurons_per_layer, activator);

                net.layout.neurons.push(Neuron::new(true, 0, &mut None));
                let neuron_buffer_address = &net.layout.neurons[0] as *const _;
                net.layout.neurons.clear();

                for layer_index in 0..neurons_per_layer.len() {
                    for _ in 0..neurons_per_layer[layer_index] {
                        let neurons_on_previous_layer = self
                            .number_of_neurons_on_previous_layer(layer_index, neurons_per_layer);
                        net.layout.neurons.push(Neuron::new(
                            false,
                            neurons_on_previous_layer,
                            &mut self.randomizer,
                        ));
                        net.layout.layers[layer_index].push(net.layout.neurons.len() - 1);
                    }

                    if layer_index != neurons_per_layer.len() - 1 {
                        net.layout.neurons.push(Neuron::new(true, 0, &mut None));
                        net.layout.layers[layer_index].push(net.layout.neurons.len() - 1);
                    }
                }
                assert_eq!(
                    &net.layout.neurons[0] as *const _, neuron_buffer_address,
                    "Reallocation of the neuron buffer detected"
                );
                net
            }
            else
            {
                panic!("Unable to build network");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::network::*;
    #[test]
    fn structure() {
        use crate::BIAS_VALUE;
        let mut randomizer = DefaultRandomizer::new();
        let neurons_per_layer = [20, 30, 10];
        let net = NetworkBuilder::new()
            .with_neurons_per_layer(&neurons_per_layer)
            .with_randomizer(&mut randomizer)
            .with_activator(|x| x)
            .build();

        let mut expected_neurons: Vec<usize> = neurons_per_layer.iter().map(|x| x + 1).collect();
        if let Some(last) = expected_neurons.last_mut() {
            *last -= 1;
        }

        assert_eq!(net.layout.layers.len(), neurons_per_layer.len());

        net.layout
            .layers
            .iter()
            .zip(expected_neurons.iter())
            .for_each(|(x, y)| assert_eq!(x.len(), *y));

        for layer_id in 0..net.layout.layers.len() - 1 {
            let layer = &net.layout.layers[layer_id];
            let last_neuron_id = *layer.last().expect("Layer empty");
            let last_neuron = &net.layout.neurons[last_neuron_id];
            assert!(relative_eq!(
                last_neuron.value.expect("Neuron w/o value"),
                BIAS_VALUE
            ));
        }

        let serialized = serde_json::to_string(&net.layout).unwrap();
        println!("{}", serialized);
    }

    #[test]
    fn calculations() {
        let mut randomizer = FixedRandomizer::new();
        let neurons_per_layer = [2, 3, 1];
        let mut net = NetworkBuilder::new()
            .with_neurons_per_layer(&neurons_per_layer)
            .with_randomizer(&mut randomizer)
            .with_activator(|x| x)
            .build();

        const INPUT_1: f64 = 3.7;
        const INPUT_2: f64 = -2.8;

        net.fire(&[INPUT_1, INPUT_2]);

        assert!(relative_eq!(
            net.layout.neurons[2].value.unwrap(),
            crate::BIAS_VALUE
        ));
        assert!(relative_eq!(
            net.layout.neurons[6].value.unwrap(),
            crate::BIAS_VALUE
        ));
        assert!(relative_eq!(
            net.layout.neurons[3].value.unwrap(),
            1.5 * INPUT_1 + 3.0 * INPUT_2 + 4.5 * 1.0
        ));
        assert!(relative_eq!(
            net.layout.neurons[4].value.unwrap(),
            6.0 * INPUT_1 + 7.5 * INPUT_2 + 9.0 * 1.0
        ));
        assert!(relative_eq!(
            net.layout.neurons[5].value.unwrap(),
            10.5 * INPUT_1 + 12.0 * INPUT_2 + 13.5 * 1.0
        ));
        assert!(relative_eq!(
            net.layout.neurons[7].value.unwrap(),
            (1.5 * INPUT_1 + 3.0 * INPUT_2 + 4.5 * 1.0) * 15.0
                + (6.0 * INPUT_1 + 7.5 * INPUT_2 + 9.0 * 1.0) * 16.5
                + (10.5 * INPUT_1 + 12.0 * INPUT_2 + 13.5 * 1.0) * 18.0
                + 19.5
        ));

        let serialized = serde_json::to_string(&net.layout).unwrap();
        println!("{}", serialized);
    }
}
