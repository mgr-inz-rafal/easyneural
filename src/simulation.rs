use crate::network::NetworkBuilder;
use crate::randomizer::DefaultRandomizer;
use crate::simulating_world::SimulatingWorld;
use crate::specimen::{Specimen, SpecimenStatus};

pub struct SimulationStatus {
    pub specimen_status: SpecimenStatus,
    pub current_tick: usize,
}

pub struct Simulation<T: SimulatingWorld> {
    pub(crate) population: Vec<Specimen>,
    pub world: Option<T>,
    parents: [Option<usize>; 2],
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
            parents: [None, None],
        })
    }

    fn is_selected_as_parent(&self, index: usize) -> bool {
        self.parents
            .iter()
            .find(|&&parent| parent == Some(index))
            .is_some()
    }

    fn add_parent_candidate(&mut self, candindate_index: usize) {
        if let Some(empty_parent) = self.parents.iter_mut().find(|parent| parent.is_none()) {
            // Replace empty parent with candidate
            *empty_parent = Some(candindate_index);
        } else {
            let worse_parent_index = match self
                .parents
                .iter()
                .enumerate()
                .min_by_key(|parent| parent.1.unwrap())
            {
                Some(worse_parent) => worse_parent.0,
                None => 0,
            };
            let candidate = &self.population[candindate_index];
            let worse_parent = &self.population[self.parents[worse_parent_index].unwrap()];
            if !self.is_selected_as_parent(candindate_index)
                && candidate.fitness > worse_parent.fitness
            {
                self.parents[worse_parent_index] = Some(candindate_index);
            }
        }
    }

    pub fn run_simulation(&mut self) {
        let mut status;
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
                        self.add_parent_candidate(specimen_index);
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
        let mut simulation =
            prepare_simulation(TEST_POPULATION_SIZE).expect("Unable to create simulation");
        simulation.add_parent_candidate(1);
        assert!(simulation.is_selected_as_parent(1));
        assert!(
            simulation.parents[1].is_none(),
            "Parent 2 should not be set here"
        );
    }

    #[test]
    fn selecting_parents_just_two() {
        const TEST_POPULATION_SIZE: usize = 5;
        let mut simulation =
            prepare_simulation(TEST_POPULATION_SIZE).expect("Unable to create simulation");
        simulation.add_parent_candidate(1);
        simulation.add_parent_candidate(2);
        assert!(simulation.is_selected_as_parent(1));
        assert!(simulation.is_selected_as_parent(2));
    }

    #[test]
    fn selecting_parents_pick_best() {
        const TEST_POPULATION_SIZE: usize = 120;
        let mut simulation =
            prepare_simulation(TEST_POPULATION_SIZE).expect("Unable to create simulation");
        for i in 0..TEST_POPULATION_SIZE {
            simulation.add_parent_candidate(i);
        }
        assert!(simulation.is_selected_as_parent(TEST_POPULATION_SIZE - 1));
        assert!(simulation.is_selected_as_parent(TEST_POPULATION_SIZE - 2));
    }

    #[test]
    fn selecting_parents_pick_best_reversed() {
        const TEST_POPULATION_SIZE: usize = 10;
        let mut simulation =
            prepare_simulation(TEST_POPULATION_SIZE).expect("Unable to create simulation");
        for i in (0..TEST_POPULATION_SIZE).rev() {
            simulation.add_parent_candidate(i);
        }

        assert!(simulation.is_selected_as_parent(TEST_POPULATION_SIZE - 1));
        assert!(simulation.is_selected_as_parent(TEST_POPULATION_SIZE - 2));
    }

    #[test]
    fn selecting_parents_overwrite_one() {
        const TEST_POPULATION_SIZE: usize = 10;
        const TEST_MIDDLE_POP: usize = TEST_POPULATION_SIZE / 2;
        let mut simulation =
            prepare_simulation(TEST_POPULATION_SIZE).expect("Unable to create simulation");
        for _ in 0..10 {
            simulation.add_parent_candidate(TEST_MIDDLE_POP);
        }
        simulation.add_parent_candidate(TEST_POPULATION_SIZE - 1);

        assert!(simulation.is_selected_as_parent(TEST_POPULATION_SIZE - 1));
        assert!(simulation.is_selected_as_parent(TEST_MIDDLE_POP));
    }

    #[test]
    fn selecting_parents_overwrite_two() {
        const TEST_POPULATION_SIZE: usize = 10;
        const TEST_MIDDLE_POP: usize = TEST_POPULATION_SIZE / 2;
        let mut simulation =
            prepare_simulation(TEST_POPULATION_SIZE).expect("Unable to create simulation");
        for _ in 0..10 {
            simulation.add_parent_candidate(TEST_MIDDLE_POP);
        }
        simulation.add_parent_candidate(TEST_POPULATION_SIZE - 1);
        simulation.add_parent_candidate(TEST_POPULATION_SIZE - 2);

        assert!(simulation.is_selected_as_parent(TEST_POPULATION_SIZE - 1));
        assert!(simulation.is_selected_as_parent(TEST_POPULATION_SIZE - 2));
    }

    #[test]
    fn selecting_parents_no_single_predominance() {
        const TEST_POPULATION_SIZE: usize = 10;
        const TEST_MIDDLE_POP: usize = TEST_POPULATION_SIZE / 2;
        const TEST_BEST_POP: usize = TEST_POPULATION_SIZE - 1;
        let mut simulation =
            prepare_simulation(TEST_POPULATION_SIZE).expect("Unable to create simulation");
        simulation.add_parent_candidate(TEST_MIDDLE_POP);
        simulation.add_parent_candidate(TEST_MIDDLE_POP);
        simulation.add_parent_candidate(TEST_BEST_POP);
        simulation.add_parent_candidate(TEST_BEST_POP);
        assert!(simulation.is_selected_as_parent(TEST_BEST_POP));
        assert!(simulation.is_selected_as_parent(TEST_MIDDLE_POP));
    }
}
