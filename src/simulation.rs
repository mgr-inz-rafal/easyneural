use crate::network::{Network, NetworkBuilder};
use crate::randomizer::DefaultRandomizer;
use crate::simulating_world::SimulatingWorld;

pub enum SpecimenStatus {
    ALIVE,
    DEAD,
}

pub struct Specimen {
    pub brain: Network,
    fitness: isize,
}

pub struct Simulation<T: SimulatingWorld> {
    pub population: Vec<Specimen>,
    pub world: T,
}

impl<T: SimulatingWorld> Simulation<T> {
    pub fn new(
        population_size: usize,
        neurons_per_layer: &[usize],
    ) -> Result<Simulation<T>, String> {
        use crate::MINIMUM_POPULATION_SIZE;

        if population_size < MINIMUM_POPULATION_SIZE {
            return Err(format!(
                "Population too small, minimum size={}",
                MINIMUM_POPULATION_SIZE
            ));
        }
        Ok(Simulation {
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
        use crate::simulating_world::SimulatingWorld;
        use crate::simulation::{Simulation, Specimen, SpecimenStatus};
        use crate::MINIMUM_POPULATION_SIZE;

        struct TestWorld;
        impl SimulatingWorld for TestWorld {
            fn new() -> TestWorld {
                TestWorld {}
            }
            fn release_specimen(&mut self, specimen: &mut Specimen) {}
            fn tick(&mut self) -> usize {
                0
            }
            fn process_inputs(&mut self, outcome: &[f64]) -> SpecimenStatus {
                SpecimenStatus::DEAD
            }
        }

        let simulation = Simulation::<TestWorld>::new(MINIMUM_POPULATION_SIZE, &[1]);
        assert!(simulation.is_ok());
        let simulation = simulation.unwrap();
        assert_eq!(simulation.population.len(), MINIMUM_POPULATION_SIZE);
    }

    #[test]
    fn population_too_small() {
        use crate::simulating_world::SimulatingWorld;
        use crate::simulation::{Simulation, Specimen, SpecimenStatus};
        use crate::MINIMUM_POPULATION_SIZE;

        struct TestWorld;
        impl SimulatingWorld for TestWorld {
            fn new() -> TestWorld {
                TestWorld {}
            }
            fn release_specimen(&mut self, specimen: &mut Specimen) {}
            fn tick(&mut self) -> usize {
                0
            }
            fn process_inputs(&mut self, outcome: &[f64]) -> SpecimenStatus {
                SpecimenStatus::DEAD
            }
        }

        let simulation = Simulation::<TestWorld>::new(MINIMUM_POPULATION_SIZE - 1, &[1]);
        assert!(simulation.is_err());
    }
}
