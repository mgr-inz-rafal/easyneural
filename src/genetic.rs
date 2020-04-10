use crate::network::NetworkLayout;
use crate::randomizer::RandomProvider;
use rand::Rng;

fn should_mutate(rng: &mut rand::rngs::ThreadRng, probability: f64) -> bool {
    if rng.gen::<f64>() < probability {
        true
    } else {
        false
    }
}

pub(crate) fn crossover(parents: &[NetworkLayout; 2]) -> [NetworkLayout; 2] {
    let crossover_point = parents[0].neurons.len() / 2;
    let (mut offspring_1, mut offspring_2) = (parents[0].clone(), parents[1].clone());
    for i in 0..crossover_point {
        std::mem::swap(&mut offspring_1.neurons[i], &mut offspring_2.neurons[i]);
    }
    [offspring_1, offspring_2]
}

pub(crate) fn mutate<'a>(
    mut parents: [NetworkLayout; 2],
    randomizer: &'a mut dyn RandomProvider,
    mutation_probability: f64,
) -> [NetworkLayout; 2] {
    let mut uniform_randomizer = rand::thread_rng();
    parents.iter_mut().for_each(|parent| {
        parent.neurons.iter_mut().for_each(|neuron| {
            neuron.inputs.iter_mut().for_each(|input| {
                if should_mutate(&mut uniform_randomizer, mutation_probability) {
                    *input = randomizer.get_number();
                }
            })
        });
    });

    parents
}

#[cfg(test)]
mod tests {
    use crate::genetic::{crossover, mutate};
    use crate::network::NetworkLayout;
    use crate::neuron::Neuron;
    use crate::randomizer::RandomProvider;

    fn create_test_pops<'a>(
        neurons: usize,
        inputs: usize,
        randomizer: &'a mut dyn RandomProvider,
    ) -> (NetworkLayout, NetworkLayout) {
        (
            NetworkLayout {
                neurons: std::iter::repeat_with(|| {
                    Neuron::new(false, inputs, &mut Some(randomizer))
                })
                .take(neurons)
                .collect(),
                layers: vec![],
            },
            NetworkLayout {
                neurons: std::iter::repeat_with(|| {
                    Neuron::new(false, inputs, &mut Some(randomizer))
                })
                .take(neurons)
                .collect(),
                layers: vec![],
            },
        )
    }

    #[test]
    fn test_crossover() {
        pub(crate) struct TestRandomizer {
            current: f64,
        }
        impl RandomProvider for TestRandomizer {
            fn get_number(&mut self) -> f64 {
                self.current += 1.0;
                self.current
            }
        }
        let mut randomizer = TestRandomizer { current: 0.0 };

        const NEURON_COUNT: usize = 5;
        const INPUT_COUNT: usize = 1;
        let (pop1, pop2) = create_test_pops(NEURON_COUNT, INPUT_COUNT, &mut randomizer);

        // Before crossover:
        //      1.0 - 2.0 - 3.0 - 4.0 -  5.0
        //      6.0 - 7.0 - 8.0 - 9.0 - 10.0
        //
        // After crossover:
        //      6.0 - 7.0 - 3.0 - 4.0 -  5.0
        //      1.0 - 2.0 - 8.0 - 9.0 - 10.0
        let [offspring_1, offspring_2] = crossover(&[pop1, pop2]);
        offspring_1
            .neurons
            .iter()
            .zip([6.0, 7.0, 3.0, 4.0, 5.0].iter())
            .for_each(|(a, b)| assert!(relative_eq!(a.inputs[0], b)));
        offspring_2
            .neurons
            .iter()
            .zip([1.0, 2.0, 8.0, 9.0, 10.0].iter())
            .for_each(|(a, b)| assert!(relative_eq!(a.inputs[0], b)));
    }

    #[test]
    fn test_mutate() {
        pub(crate) struct TestRandomizer {
            current: f64,
        }
        impl RandomProvider for TestRandomizer {
            fn get_number(&mut self) -> f64 {
                self.current
            }
        }
        let mut randomizer = TestRandomizer { current: 100.0 };

        pub(crate) struct MutationRandomizer {
            current: f64,
        }
        impl RandomProvider for MutationRandomizer {
            fn get_number(&mut self) -> f64 {
                self.current
            }
        }
        const MUTATED_VALUE: f64 = -100.0;
        let mut mutation_randomizer = TestRandomizer {
            current: MUTATED_VALUE,
        };

        const NEURON_COUNT: usize = 50;
        const INPUT_COUNT: usize = 150;
        const TOTAL_INPUTS: usize = NEURON_COUNT * INPUT_COUNT * 2;
        const MUTATION_PROBABILITY: f64 = 0.5;
        let (pop1, pop2) = create_test_pops(NEURON_COUNT, INPUT_COUNT, &mut randomizer);

        let mutated = mutate([pop1, pop2], &mut mutation_randomizer, MUTATION_PROBABILITY);
        let mut counter = 0;
        mutated.iter().for_each(|pop| {
            pop.neurons.iter().for_each(|neuron| {
                counter += neuron
                    .inputs
                    .iter()
                    .filter(|input| relative_eq!(**input, MUTATED_VALUE))
                    .count()
            });
        });

        let percentage_mutated: f64 = counter as f64 / TOTAL_INPUTS as f64;
        const TOLERANCE: f64 = 0.10; // Allow tolerance, since we use real randomizer while mutating
        assert!(relative_eq!(
            percentage_mutated,
            MUTATION_PROBABILITY,
            epsilon = TOLERANCE
        ))
    }
}
