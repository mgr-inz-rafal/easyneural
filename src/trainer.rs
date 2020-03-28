use crate::network::{Network, NetworkBuilder};
use crate::randomizer::DefaultRandomizer;
use crate::world::World;

pub struct Specimen {
    pub brain: Network,
    fitness: isize,
}

pub struct Trainer<T: World> {
    pub population: Vec<Specimen>,
    pub world: T,
}

impl<T: World> Trainer<T> {
    pub fn new(population_size: usize, neurons_per_layer: &[usize]) -> Result<Trainer<T>, String> {
        use crate::MINIMUM_POPULATION_SIZE;

        if population_size < MINIMUM_POPULATION_SIZE {
            return Err(format!(
                "Population too small, minimum size={}",
                MINIMUM_POPULATION_SIZE
            ));
        }
        Ok(Trainer {
            world: T::new(),
            population: std::iter::repeat_with(|| {
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

    pub fn run_simulation(&mut self) {
        for mut specimen in &mut self.population {
            self.world.release_specimen(&mut specimen);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_population_size() {
        use crate::trainer::{Specimen, Trainer};
        use crate::world::World;
        use crate::MINIMUM_POPULATION_SIZE;

        struct TestWorld;
        impl World for TestWorld {
            fn new() -> TestWorld {
                TestWorld {}
            }
            fn release_specimen(&mut self, specimen: &mut Specimen) {}
        }

        let trainer = Trainer::<TestWorld>::new(MINIMUM_POPULATION_SIZE, &[1]);
        assert!(trainer.is_ok());
        let trainer = trainer.unwrap();
        assert_eq!(trainer.population.len(), MINIMUM_POPULATION_SIZE);
    }

    #[test]
    fn population_too_small() {
        use crate::trainer::{Specimen, Trainer};
        use crate::world::World;
        use crate::MINIMUM_POPULATION_SIZE;

        struct TestWorld;
        impl World for TestWorld {
            fn new() -> TestWorld {
                TestWorld {}
            }
            fn release_specimen(&mut self, specimen: &mut Specimen) {}
        }

        let trainer = Trainer::<TestWorld>::new(MINIMUM_POPULATION_SIZE - 1, &[1]);
        assert!(trainer.is_err());
    }
}
