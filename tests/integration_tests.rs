extern crate approx;
extern crate easyneural;
use easyneural::randomizer::DefaultRandomizer;
use easyneural::simulating_world::SimulatingWorld;
use easyneural::simulation::{Finish, Simulation, SimulationStatus};
use easyneural::specimen::SpecimenStatus;

use if_chain::if_chain;

struct MyWorld {
    tick: usize,
    liveliness: isize,
}

impl MyWorld {
    fn get_specimen_score(&self, foo_data: f64) -> f64 {
        self.tick as f64 / foo_data // TODO: Just some made-up formula for now
    }
}

impl SimulatingWorld for MyWorld {
    fn new() -> MyWorld {
        MyWorld {
            tick: 0,
            liveliness: 0,
        }
    }

    fn tick(&mut self, input: &[f64]) -> SimulationStatus {
        self.tick += 1;

        self.liveliness = self.liveliness + if input[0] < 0.5 { -3 } else { 1 };
        let alive_status;
        match self.liveliness {
            -5..=5 => alive_status = SpecimenStatus::ALIVE(self.get_specimen_score(input[0])),
            _ => alive_status = SpecimenStatus::DEAD(self.get_specimen_score(input[0])),
        }

        SimulationStatus {
            specimen_status: alive_status,
            current_tick: self.tick,
        }
    }

    fn get_world_state(&self) -> Vec<f64> {
        vec![
            // TODO: For now, simulate some basic feedback from specimen
            self.get_specimen_score(-21.0),
            self.get_specimen_score(21.0),
        ]
    }
}

#[test]
fn test_run_training_session() {
    const POPULATION_SIZE: usize = 10;
    const SIMULATION_ROUNDS: usize = 10;

    let neurons_per_layer = [2, 4, 5, 1];
    let mut randomizer = DefaultRandomizer::new();

    if_chain! {
        if let Ok(mut session) = Simulation::<MyWorld>::new(POPULATION_SIZE, &neurons_per_layer, &mut randomizer, None);
        if let Ok(_) = session.run(Finish::Occurences(SIMULATION_ROUNDS));
        then {
            assert_eq!(session.get_number_of_iterations(), SIMULATION_ROUNDS);
        }
        else
        {
            assert!(false, "Error running session");
        }
    }
}
