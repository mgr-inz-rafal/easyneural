extern crate easyneural;

use easyneural::simulating_world::SimulatingWorld;
use easyneural::trainer::{Specimen, Trainer};

enum SpecimenStatus {
    ALIVE,
    DEAD,
}

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

    fn release_specimen(&mut self, specimen: &mut Specimen) {
        self.tick = 0;
        loop {
            specimen.brain.fire(&[1.0, 2.0]);
            let outcome = specimen.brain.get_output();
            if let SpecimenStatus::DEAD = self.process_inputs(&outcome) {
                println!("Specimen died at tick {}", self.tick);
                break;
            }
        }
    }
}

impl MyWorld {
    fn process_inputs(&mut self, inputs: &[f64]) -> SpecimenStatus {
        self.tick += 1;
        self.liveliness = self.liveliness + if inputs[0] < 0.5 { -1 } else { 1 };
        match self.liveliness {
            -5..=5 => SpecimenStatus::ALIVE,
            _ => SpecimenStatus::DEAD,
        }
    }
}

#[test]
fn test_run_training_session() -> Result<(), String> {
    const POPULATION_SIZE: usize = 10;

    let neurons_per_layer = [2, 4, 5, 1];
    let mut session = Trainer::<MyWorld>::new(POPULATION_SIZE, &neurons_per_layer)?;
    session.run_simulation();

    Ok(())
}
