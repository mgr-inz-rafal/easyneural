use crate::trainer::{Specimen, SpecimenStatus};

pub trait SimulatingWorld {
    fn new() -> Self;
    fn tick(&mut self) -> usize;
    fn process_inputs(&mut self, outcome: &[f64]) -> SpecimenStatus;
    fn release_specimen(&mut self, specimen: &mut Specimen) {
        loop {
            let current_tick = self.tick();
            specimen.brain.fire(&[1.0, 2.0]);
            let outcome = specimen.brain.get_output();
            if let SpecimenStatus::DEAD = self.process_inputs(&outcome) {
                println!("Specimen died in tick {}", current_tick);
                break;
            }
        }
    }
}
