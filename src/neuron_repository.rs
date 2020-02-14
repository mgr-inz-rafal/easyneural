use super::neuron::{InputKind, Neuron};
use crate::NeuronValue;
pub struct NeuronRepository {
    pub(crate) neurons: Vec<Neuron>,
}

impl NeuronRepository {
    pub(crate) fn new(neuron_count: usize) -> NeuronRepository {
        NeuronRepository {
            neurons: Vec::with_capacity(neuron_count),
        }
    }

    pub(crate) fn fire(&mut self, index: usize) {
        let mut sum = 0.0;

        // TODO: This solution with two separate loops is a dirty hack, rethink this
        for input in &mut self.neurons[index].inputs {
            match input {
                InputKind::Value(cb) => {
                    let my_value = (cb.as_mut().unwrap())();
                    println!("\t\tValue: {}", my_value);
                    sum += my_value;
                }
                _ => {}
            }
        }

        for input in &self.neurons[index].inputs {
            match input {
                InputKind::Axon(axon) => {
                    let my_weight = axon.get_weight();
                    let connecting_id = axon.get_id();
                    if let Some(connecting_value) = &self.neurons[connecting_id].value {
                        println!(
                            "\t\tAxon: weight: {}, connecting_id: {}, connecting_value: {}",
                            my_weight,
                            connecting_id,
                            connecting_value.get()
                        );
                        sum += my_weight * connecting_value.get();
                    } else {
                        // TODO: Handle error here!
                    }
                }
                _ => {}
            }
        }

        if let Some(value) = &mut self.neurons[index].value {
            value.set(sum);
        } else {
            self.neurons[index].value = Some(Box::new(NeuronValue { value: sum }));
        }
    }
}
