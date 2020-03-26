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
    pub fn new(population_size: usize, neurons_per_layer: &[usize]) -> Result<Trainer, String> {
        use crate::MINIMUM_POPULATION_SIZE;

        if population_size < MINIMUM_POPULATION_SIZE {
            return Err(format!(
                "Population too small, minimum size={}",
                MINIMUM_POPULATION_SIZE
            ));
        }
        Ok(Trainer {
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
        })
    }

    pub fn run_session(&self, inputs: &[f64]) {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_population_size() {
        use crate::trainer::Trainer;
        use crate::MINIMUM_POPULATION_SIZE;

        let trainer = Trainer::new(MINIMUM_POPULATION_SIZE, &[1]);
        assert!(trainer.is_ok());
        let trainer = trainer.unwrap();
        assert_eq!(trainer.population.len(), MINIMUM_POPULATION_SIZE);
    }

    #[test]
    fn population_too_small() {
        use crate::trainer::Trainer;
        use crate::MINIMUM_POPULATION_SIZE;

        let trainer = Trainer::new(MINIMUM_POPULATION_SIZE - 1, &[1]);
        assert!(trainer.is_err());
    }
}
