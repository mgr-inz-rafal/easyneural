use std::time::Duration;

use crate::genetic::{crossover, mutate};
use crate::network::NetworkBuilder;
use crate::randomizer::RandomProvider;
use crate::simulating_world::SimulatingWorld;
use crate::specimen::{Specimen, SpecimenStatus};

const DEFAULT_MUTATION_PROBABILITY: f64 = 0.1;

pub enum Finish {
    Occurences(usize),
    Timeout(Duration),
}

pub struct SimulationStatus {
    pub specimen_status: SpecimenStatus,
    pub current_tick: usize,
}

pub struct Simulation<'a, T: SimulatingWorld> {
    pub(crate) population: Vec<Specimen>,
    pub world: Option<T>,
    parents: [Option<usize>; 2],
    randomizer: Option<&'a mut dyn RandomProvider>,
    mutation_probability: f64,

    // TODO: Temporary - will be reworked with SimulationStatus
    counter: usize,
}

impl<'a, T: SimulatingWorld> Simulation<'a, T> {
    pub fn new(
        population_size: usize,
        neurons_per_layer: &[usize],
        randomizer: &'a mut dyn RandomProvider,
        mutation_probability: Option<f64>,
    ) -> Result<Simulation<'a, T>, String> {
        use crate::MINIMUM_POPULATION_SIZE;

        if population_size < MINIMUM_POPULATION_SIZE {
            return Err(format!(
                "Population too small, minimum size={}",
                MINIMUM_POPULATION_SIZE
            ));
        }

        if population_size % 2 != 0 {
            return Err("Population size must be an even number".to_string());
        };

        Ok(Simulation {
            world: None,
            population: std::iter::repeat_with(|| {
                NetworkBuilder::new()
                    .with_neurons_per_layer(&neurons_per_layer)
                    .with_randomizer(randomizer)
                    .build()
            })
            .take(population_size)
            .map(|network| Specimen {
                brain: network,
                fitness: 0.0,
            })
            .collect(),
            parents: [None, None],
            randomizer: Some(randomizer),
            mutation_probability: mutation_probability.unwrap_or(DEFAULT_MUTATION_PROBABILITY),
            counter: 0,
        })
    }

    pub fn spawn_new_population_using(&mut self, parents: &[crate::Specimen; 2]) {
        for i in 0..self.population.len() / 2 {
            let evolved = self.evolve(&parents);
            self.population[i * 2].brain.layout = evolved[0].brain.clone();
            self.population[i * 2].fitness = 0.0;
            self.population[i * 2 + 1].brain.layout = evolved[1].brain.clone();
            self.population[i * 2 + 1].fitness = 0.0;
        }
    }

    pub fn evolve(&mut self, parents: &[crate::Specimen; 2]) -> [crate::Specimen; 2] {
        mutate(
            crossover(&parents),
            self.randomizer.as_deref_mut().unwrap(),
            self.mutation_probability,
        )
    }

    fn simulation_loop(&mut self) -> Result<[crate::Specimen; 2], String> {
        self.counter += 1;
        let best_pops = self.simulate()?;

        // TODO: spawn_new_population_using() is not tested
        // TODO: Do not call spawn_new_population_using() if it is the last iteration of the simulation_loop
        self.spawn_new_population_using(&best_pops);
        Ok(best_pops)
    }

    pub fn run(&mut self, finish: Finish) -> Result<[crate::Specimen; 2], String> {
        match finish {
            Finish::Occurences(count) => {
                let mut best_parents_so_far = None;
                for _ in 0..count {
                    match self.simulation_loop() {
                        Err(message) => return Err(message),
                        Ok(best_parents) => {
                            best_parents_so_far = Some(best_parents);
                        }
                    }
                }
                if let Some(best_parents) = best_parents_so_far {
                    return Ok(best_parents);
                } else {
                    return Err(
                        "Simulation finished, but no best parents could be selected".to_string()
                    );
                }
            }
            _ => return Err("Simulation end trigger not supported yet".to_string()),
        };
    }

    pub fn get_number_of_iterations(&self) -> usize {
        self.counter
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

    pub fn simulate(&mut self) -> Result<[crate::Specimen; 2], String> {
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
                        specimen.fitness = fitness;
                        self.add_parent_candidate(specimen_index);
                        break;
                    }
                    current_state = world.get_world_state();
                }
            }
        }

        if self
            .parents
            .iter_mut()
            .find(|parent| parent.is_none())
            .is_some()
        {
            return Err(
                "Simulation finished w/o nominating best parents. This is a bug, please report"
                    .to_string(),
            );
        }

        Ok([
            crate::Specimen {
                brain: self.population[self.parents[0].unwrap()]
                    .brain
                    .layout
                    .clone(),
                fitness: self.population[self.parents[0].unwrap()].fitness,
            },
            crate::Specimen {
                brain: self.population[self.parents[1].unwrap()]
                    .brain
                    .layout
                    .clone(),
                fitness: self.population[self.parents[1].unwrap()].fitness,
            },
        ])
    }
}

#[cfg(test)]
mod tests {
    use crate::network::NetworkLayout;
    use crate::randomizer::{DefaultRandomizer, RandomProvider};
    use crate::simulation::{
        Finish, SimulatingWorld, Simulation, SimulationStatus, SpecimenStatus,
    };
    use crate::MINIMUM_POPULATION_SIZE;
    use if_chain::if_chain;

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
            vec![1.0, -1.0]
        }
    }

    fn prepare_simulation<'a>(
        population_size: usize,
        randomizer: &'a mut dyn RandomProvider,
    ) -> Option<Simulation<'a, TestWorld>> {
        let simulation =
            Simulation::<TestWorld>::new(population_size, &[2, 3, 4, 5, 1], randomizer, None);
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

    fn get_all_neuron_inputs_sum(network: &NetworkLayout) -> f64 {
        network
            .neurons
            .iter()
            .map(|neuron| &neuron.inputs)
            .flatten()
            .sum::<f64>()
    }

    #[test]
    fn check_population_size() {
        let mut randomizer = DefaultRandomizer::new();
        let simulation = prepare_simulation(MINIMUM_POPULATION_SIZE, &mut randomizer)
            .expect("Unable to create simulation");
        assert_eq!(simulation.population.len(), MINIMUM_POPULATION_SIZE);
    }

    #[test]
    fn population_too_small() {
        let mut randomizer = DefaultRandomizer::new();
        let simulation = prepare_simulation(MINIMUM_POPULATION_SIZE - 1, &mut randomizer);
        assert!(simulation.is_none());
    }

    #[test]
    fn evolved_parents_are_different() {
        let mut randomizer = DefaultRandomizer::new();
        let simulation = prepare_simulation(MINIMUM_POPULATION_SIZE, &mut randomizer);
        if_chain! {
            if let Some(mut simulation) = simulation;
            if let Ok(best_specimen) = simulation.run(Finish::Occurences(2));
            then {
                assert!(relative_ne!(
                    get_all_neuron_inputs_sum(&best_specimen[0].brain),
                    get_all_neuron_inputs_sum(&best_specimen[1].brain)
                ))
            }
            else{
                assert!(false);
            }
        }
    }

    #[test]
    fn selecting_parents_just_one() {
        const TEST_POPULATION_SIZE: usize = 6;
        let mut randomizer = DefaultRandomizer::new();
        let mut simulation = prepare_simulation(TEST_POPULATION_SIZE, &mut randomizer)
            .expect("Unable to create simulation");
        simulation.add_parent_candidate(1);
        assert!(simulation.is_selected_as_parent(1));
        assert!(
            simulation.parents[1].is_none(),
            "Parent 2 should not be set here"
        );
    }

    #[test]
    fn selecting_parents_just_two() {
        const TEST_POPULATION_SIZE: usize = 6;
        let mut randomizer = DefaultRandomizer::new();
        let mut simulation = prepare_simulation(TEST_POPULATION_SIZE, &mut randomizer)
            .expect("Unable to create simulation");
        simulation.add_parent_candidate(1);
        simulation.add_parent_candidate(2);
        assert!(simulation.is_selected_as_parent(1));
        assert!(simulation.is_selected_as_parent(2));
    }

    #[test]
    fn selecting_parents_pick_best() {
        const TEST_POPULATION_SIZE: usize = 120;
        let mut randomizer = DefaultRandomizer::new();
        let mut simulation = prepare_simulation(TEST_POPULATION_SIZE, &mut randomizer)
            .expect("Unable to create simulation");
        for i in 0..TEST_POPULATION_SIZE {
            simulation.add_parent_candidate(i);
        }
        assert!(simulation.is_selected_as_parent(TEST_POPULATION_SIZE - 1));
        assert!(simulation.is_selected_as_parent(TEST_POPULATION_SIZE - 2));
    }

    #[test]
    fn selecting_parents_pick_best_reversed() {
        const TEST_POPULATION_SIZE: usize = 10;
        let mut randomizer = DefaultRandomizer::new();
        let mut simulation = prepare_simulation(TEST_POPULATION_SIZE, &mut randomizer)
            .expect("Unable to create simulation");
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
        let mut randomizer = DefaultRandomizer::new();
        let mut simulation = prepare_simulation(TEST_POPULATION_SIZE, &mut randomizer)
            .expect("Unable to create simulation");
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
        let mut randomizer = DefaultRandomizer::new();
        let mut simulation = prepare_simulation(TEST_POPULATION_SIZE, &mut randomizer)
            .expect("Unable to create simulation");
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
        let mut randomizer = DefaultRandomizer::new();
        let mut simulation = prepare_simulation(TEST_POPULATION_SIZE, &mut randomizer)
            .expect("Unable to create simulation");
        simulation.add_parent_candidate(TEST_MIDDLE_POP);
        simulation.add_parent_candidate(TEST_MIDDLE_POP);
        simulation.add_parent_candidate(TEST_BEST_POP);
        simulation.add_parent_candidate(TEST_BEST_POP);
        assert!(simulation.is_selected_as_parent(TEST_BEST_POP));
        assert!(simulation.is_selected_as_parent(TEST_MIDDLE_POP));
    }
}
