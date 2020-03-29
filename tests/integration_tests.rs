extern crate easyneural;

use easyneural::simulating_world::SimulatingWorld;
use easyneural::simulation::{Simulation, SimulationStatus, SpecimenStatus};

struct MyWorld {
    tick: usize,
    liveliness: isize,
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

        self.liveliness = self.liveliness + if input[0] < 0.5 { -1 } else { 1 };
        let alive_status;
        match self.liveliness {
            -5..=5 => alive_status = SpecimenStatus::ALIVE,
            _ => alive_status = SpecimenStatus::DEAD,
        }

        SimulationStatus {
            specimen_status: alive_status,
            current_tick: self.tick,
        }
    }

    fn get_world_state(&self) -> Vec<f64> {
        vec![-1.0, -1.0] // TODO: Current input values
    }
}

#[test]
fn test_run_training_session() -> Result<(), String> {
    const POPULATION_SIZE: usize = 10;

    let neurons_per_layer = [2, 4, 5, 1];
    let mut session = Simulation::<MyWorld>::new(POPULATION_SIZE, &neurons_per_layer)?;
    session.run_simulation();

    Ok(())
}
