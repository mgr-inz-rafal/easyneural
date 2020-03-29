use crate::network::{Network, NetworkBuilder};
use crate::randomizer::DefaultRandomizer;
use crate::simulating_world::SimulatingWorld;

pub enum SpecimenStatus {
    ALIVE,
    DEAD,
}

pub struct SimulationStatus {
    pub specimen_status: SpecimenStatus,
    pub current_tick: usize,
}

// TODO: To separate module
pub struct Specimen {
    pub brain: Network,
    fitness: isize, // TODO: Move to SpecimenStatus
}

impl Specimen {
    fn tick(&mut self, world_input: &[f64]) {
        self.brain.fire(world_input);
    }
}

pub struct Simulation<T: SimulatingWorld> {
    pub population: Vec<Specimen>,
    pub world: Option<T>,
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
            world: None,
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
        let mut status;
        for specimen in &mut self.population {
            self.world = Some(T::new());
            if let Some(world) = &mut self.world {
                let mut current_state = world.get_world_state();
                loop {
                    specimen.tick(&current_state);
                    let output = specimen.brain.get_output();
                    status = world.tick(&output);
                    if let SpecimenStatus::DEAD = status.specimen_status {
                        break;
                    }
                    current_state = world.get_world_state();
                }
                println!("Specimen died in tick {}", status.current_tick);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::simulation::{SimulatingWorld, SimulationStatus, SpecimenStatus};

    struct TestWorld;
    impl SimulatingWorld for TestWorld {
        fn new() -> TestWorld {
            TestWorld {}
        }
        fn tick(&mut self, _: &[f64]) -> SimulationStatus {
            SimulationStatus {
                specimen_status: SpecimenStatus::DEAD,
                current_tick: 0,
            }
        }
        fn get_world_state(&self) -> Vec<f64> {
            vec![]
        }
    }

    #[test]
    fn check_population_size() {
        use crate::simulation::Simulation;
        use crate::MINIMUM_POPULATION_SIZE;

        let simulation = Simulation::<TestWorld>::new(MINIMUM_POPULATION_SIZE, &[1]);
        assert!(simulation.is_ok());
        let simulation = simulation.unwrap();
        assert_eq!(simulation.population.len(), MINIMUM_POPULATION_SIZE);
    }

    #[test]
    fn population_too_small() {
        use crate::simulation::Simulation;
        use crate::MINIMUM_POPULATION_SIZE;

        let simulation = Simulation::<TestWorld>::new(MINIMUM_POPULATION_SIZE - 1, &[1]);
        assert!(simulation.is_err());
    }
}
