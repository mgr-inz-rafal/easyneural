use crate::network::{Network, NetworkBuilder};
use crate::randomizer::DefaultRandomizer;

struct Specimen {
    brain: Network,
    fitness: isize,
}

pub struct Trainer {
    population: Vec<Specimen>,
}

impl Trainer {
    pub fn new(population_size: usize, neurons_per_layer: &[usize]) -> Trainer {
        Trainer {
            population: std::iter::repeat({
                NetworkBuilder::new()
                    .with_neurons_per_layer(&neurons_per_layer)
                    .with_randomizer(&mut DefaultRandomizer::new())
                    .build()
            })
            .take(population_size)
            .map(|network| Specimen {
                brain: network,
                fitness: 0,
            })
            .collect(),
        }
    }

    pub fn run_session(&self, inputs: &[f64]) {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_population_size() {
        use crate::trainer::Trainer;
        const POPULATION_SIZE: usize = 10;

        let trainer = Trainer::new(POPULATION_SIZE, &[1]);

        assert_eq!(trainer.population.len(), POPULATION_SIZE);
    }
}
