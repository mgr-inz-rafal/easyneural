use std::time::Duration;

use crate::genetic::{crossover, mutate};
use crate::network::NetworkBuilder;
use crate::randomizer::RandomProvider;
use crate::simulating_world::SimulatingWorld;
use crate::specimen::{Specimen, SpecimenStatus};

const DEFAULT_MUTATION_PROBABILITY: f64 = 0.1;

/// Finish condition for the learning session.
pub enum Finish {
    /// Learning will stop after given number of iterations.
    Occurences(usize),

    /// **[NOT IMPLEMENTED YET]** Learning will stop after specified time.
    Timeout(Duration),
}

/// Represents simulation status.
pub struct SimulationStatus {
    pub specimen_status: SpecimenStatus,
    pub current_tick: usize,
}

/// Main struct that handles the learning logic.
pub struct Simulation<'a, T: SimulatingWorld> {
    pub(crate) population: Vec<Specimen>,
    pub(crate) world: Option<T>,
    parents: Vec<(usize, f64)>,
    randomizer: Option<&'a mut dyn RandomProvider>,
    mutation_probability: f64,

    // TODO: Temporary - will be reworked with SimulationStatus
    counter: usize,
}

impl<'a, T: SimulatingWorld> Simulation<'a, T> {
    /// Creates new simulation struct.
    ///
    /// `mutation_probability` is represented as float number from range `[0.0, 1.0)`.
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
            parents: vec![],
            randomizer: Some(randomizer),
            mutation_probability: mutation_probability.unwrap_or(DEFAULT_MUTATION_PROBABILITY),
            counter: 0,
        })
    }

    pub(crate) fn evolve_population(&mut self, parents: &[crate::Specimen; 2]) {
        self.parents.clear();
        for i in 0..self.population.len() / 2 {
            let evolved = self.evolve(&parents);
            self.population[i * 2].brain.layout = evolved[0].brain.clone();
            self.population[i * 2].fitness = 0.0;
            self.population[i * 2 + 1].brain.layout = evolved[1].brain.clone();
            self.population[i * 2 + 1].fitness = 0.0;
        }
    }

    pub(crate) fn evolve(&mut self, parents: &[crate::Specimen; 2]) -> [crate::Specimen; 2] {
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
        self.evolve_population(&best_pops);
        Ok(best_pops)
    }

    /// Runs the learning round by using the specified specimen
    /// as parents for new generation.
    ///
    /// Use this function if you already have some trained networks (represented
    /// by `Specimen`) and would like to use them as a base
    /// for further learning
    pub fn run_with_parents(
        &mut self,
        finish: Finish,
        parents: [crate::Specimen; 2],
    ) -> Result<[crate::Specimen; 2], String> {
        self.evolve_population(&parents);
        self.run(finish)
    }

    /// Runs the learning round.
    ///
    /// Returns two best specimen from the most recent generation. These specimen
    /// might be used to resume training by using [`run_with_parents`](#method.run_with_parents).
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
                    Ok(best_parents)
                } else {
                    Err("Simulation finished, but no best parents could be selected".to_string())
                }
            }
            _ => Err("Simulation end trigger not supported yet".to_string()),
        }
    }

    /// Returns number of iterations used in recent learning session.
    pub fn get_number_of_iterations(&self) -> usize {
        self.counter
    }

    fn add_parent_candidate(&mut self, candindate_index: usize) {
        self.parents
            .push((candindate_index, self.population[candindate_index].fitness));
    }

    fn simulate(&mut self) -> Result<[crate::Specimen; 2], String> {
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

        self.parents
            .sort_by(|b, a| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        Ok([
            crate::Specimen {
                brain: self.population[self.parents[0].0].brain.layout.clone(),
                fitness: self.parents[0].1,
            },
            crate::Specimen {
                brain: self.population[self.parents[1].0].brain.layout.clone(),
                fitness: self.parents[1].1,
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

    impl<'a, T: SimulatingWorld> Simulation<'a, T> {
        pub(crate) fn is_selected_as_parent(&self, index: usize) -> bool {
            self.parents.iter().any(|&parent| parent.0 == index)
        }
    }

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
