use crate::network::{Network, NetworkBuilder};
use crate::randomizer::DefaultRandomizer;
use crate::simulating_world::SimulatingWorld;

pub enum SpecimenStatus {
    ALIVE,
    DEAD(f64),
}

pub struct SimulationStatus {
    pub specimen_status: SpecimenStatus,
    pub current_tick: usize,
}

// TODO: To separate module
pub struct Specimen {
    pub brain: Network,
    fitness: f64, // TODO: Move to SpecimenStatus
}

impl Specimen {
    fn tick(&mut self, world_input: &[f64]) -> Vec<f64> {
        self.brain.fire(world_input);
        self.brain.get_output()
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
                fitness: 0.0,
            })
            .collect(),
        })
    }

    fn add_parent_candidate(&mut self, index: usize, parents: &mut [Option<usize>]) {
        if let Some(empty_parent) = parents.iter_mut().find(|parent| parent.is_none()) {
            *empty_parent = Some(index);
        } else {
            // TODO: Note that in the future there might be more parents,
            // for example, one might want to crossbreed more than 2 best specimens.
            let worse_parent =
                if self.population[parents[0].expect("Parent 0 should be existent here")].fitness
                    < self.population[parents[1].expect("Parent 0 should be existent here")].fitness
                {
                    0
                } else {
                    1
                };
            parents[worse_parent] = Some(index);
        }
    }

    pub fn run_simulation(&mut self) {
        let mut status;
        let mut parents: [Option<usize>; 2] = [None, None];
        for specimen_index in 0..self.population.len() {
            let specimen = &mut self.population[specimen_index];
            self.world = Some(T::new());
            if let Some(world) = &mut self.world {
                let mut current_state = world.get_world_state();
                loop {
                    let output = specimen.tick(&current_state);
                    status = world.tick(&output);
                    if let SpecimenStatus::DEAD(fitness) = status.specimen_status {
                        println!(
                            "Specimen died in tick {} with fitness {}",
                            status.current_tick, fitness
                        );
                        specimen.fitness = fitness;
                        self.add_parent_candidate(specimen_index, &mut parents);
                        break;
                    }
                    current_state = world.get_world_state();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::simulation::{SimulatingWorld, Simulation, SimulationStatus, SpecimenStatus};
    use crate::MINIMUM_POPULATION_SIZE;

    struct TestWorld;
    impl SimulatingWorld for TestWorld {
        fn new() -> TestWorld {
            TestWorld {}
        }
        fn tick(&mut self, _: &[f64]) -> SimulationStatus {
            SimulationStatus {
                specimen_status: SpecimenStatus::DEAD(-1.0),
                current_tick: 0,
            }
        }
        fn get_world_state(&self) -> Vec<f64> {
            vec![]
        }
    }

    #[test]
    fn check_population_size() {
        let simulation = Simulation::<TestWorld>::new(MINIMUM_POPULATION_SIZE, &[1]);
        assert!(simulation.is_ok());
        let simulation = simulation.unwrap();
        assert_eq!(simulation.population.len(), MINIMUM_POPULATION_SIZE);
    }

    #[test]
    fn population_too_small() {
        let simulation = Simulation::<TestWorld>::new(MINIMUM_POPULATION_SIZE - 1, &[1]);
        assert!(simulation.is_err());
    }

    fn prepare_simulation(population_size: usize) -> Option<Simulation<TestWorld>> {
        let simulation = Simulation::<TestWorld>::new(population_size, &[1]);
        if let Ok(mut simulation) = simulation {
            simulation
                .population
                .iter_mut()
                .enumerate()
                .for_each(|(index, pop)| pop.fitness = (index * 2) as f64);
            return Some(simulation);
        }
        None
    }

    #[test]
    fn selecting_parents_just_one() {
        const TEST_POPULATION_SIZE: usize = 5;
        let mut parents: [Option<usize>; 2] = [None, None];
        let mut simulation =
            prepare_simulation(TEST_POPULATION_SIZE).expect("Unable to create simulation");
        simulation.add_parent_candidate(1, &mut parents);
        assert_eq!(parents[0].expect("Parent 1 not set correctly"), 1);
        assert!(parents[1].is_none(), "Parent 2 should not be set here");
    }

    #[test]
    fn selecting_parents_just_two() {
        const TEST_POPULATION_SIZE: usize = 5;
        let mut parents: [Option<usize>; 2] = [None, None];
        let mut simulation =
            prepare_simulation(TEST_POPULATION_SIZE).expect("Unable to create simulation");
        simulation.add_parent_candidate(1, &mut parents);
        simulation.add_parent_candidate(2, &mut parents);
        assert_eq!(parents[0].expect("Parent 1 not set correctly"), 1);
        assert_eq!(parents[1].expect("Parent 2 not set correctly"), 2);
    }

    #[test]
    fn selecting_parents_pick_best() {
        const TEST_POPULATION_SIZE: usize = 5;
        let mut parents: [Option<usize>; 2] = [None, None];
        let mut simulation =
            prepare_simulation(TEST_POPULATION_SIZE).expect("Unable to create simulation");
        for i in 0..TEST_POPULATION_SIZE {
            simulation.add_parent_candidate(i, &mut parents);
        }
        assert_eq!(parents[0].expect("Parent 1 not set correctly"), 4);
        assert_eq!(parents[1].expect("Parent 2 not set correctly"), 3);
    }
}
