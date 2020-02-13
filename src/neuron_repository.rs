use super::neuron::{InputKind, Neuron};

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
        if let Some(fixed_value) = self.neurons[index].fixed_value {
            self.neurons[index].value = Some(fixed_value);
        }

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
                    let connecting_value = &self.neurons[connecting_id].value;
                    println!(
                        "\t\tAxon: weight: {}, connecting_id: {}, connecting_value: {}",
                        my_weight,
                        connecting_id,
                        connecting_value.unwrap()
                    );
                    sum += my_weight * connecting_value.unwrap();
                }
                _ => {}
            }
        }

        self.neurons[index].value = Some(sum);
    }
}
