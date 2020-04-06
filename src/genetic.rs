use crate::network::NetworkLayout;

fn crossover(parents: [&NetworkLayout; 2]) -> [NetworkLayout; 2] {
    let crossover_point = parents[0].neurons.len() / 2;
    let (mut offspring_1, mut offspring_2) = (parents[0].clone(), parents[1].clone());
    for i in 0..crossover_point {
        std::mem::swap(&mut offspring_1.neurons[i], &mut offspring_2.neurons[i]);
    }
    [offspring_1, offspring_2]
}

pub fn mutate(parents: [&NetworkLayout; 2]) {
    let mixed_genes = crossover(parents);
}

#[cfg(test)]
mod tests {
    use crate::genetic::crossover;
    use crate::network::NetworkLayout;
    use crate::neuron::Neuron;
    use crate::randomizer::RandomProvider;

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
        let pop1 = NetworkLayout {
            neurons: std::iter::repeat_with(|| Neuron::new(false, 1, &mut Some(&mut randomizer)))
                .take(NEURON_COUNT)
                .collect(),
            layers: vec![],
        };
        let pop2 = NetworkLayout {
            neurons: std::iter::repeat_with(|| Neuron::new(false, 1, &mut Some(&mut randomizer)))
                .take(NEURON_COUNT)
                .collect(),
            layers: vec![],
        };

        // Before crossover:
        //      1.0 - 2.0 - 3.0 - 4.0 -  5.0
        //      6.0 - 7.0 - 8.0 - 9.0 - 10.0
        //
        // After crossover:
        //      6.0 - 7.0 - 3.0 - 4.0 -  5.0
        //      1.0 - 2.0 - 8.0 - 9.0 - 10.0
        let [offspring_1, offspring_2] = crossover([&pop1, &pop2]);
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
}
