extern crate easyneural;

use easyneural::trainer::{Specimen, Trainer};

enum SpecimenStatus {
    ALIVE,
    DEAD,
}

struct World {
    tick: usize,
    liveliness: isize,
}

impl World {
    pub fn new() -> World {
        World {
            tick: 0,
            liveliness: 0,
        }
    }

    pub fn drop(&mut self, specimen: &mut Specimen) {
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

    let mut world = World::new();

    let neurons_per_layer = [2, 4, 5, 1];
    let mut session = Trainer::new(POPULATION_SIZE, &neurons_per_layer)?;

    session.population.iter_mut().for_each(|specimen| {
        world.drop(specimen);
    });

    Ok(())
}
