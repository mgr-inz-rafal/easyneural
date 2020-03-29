use crate::simulation::SimulationStatus;

pub trait SimulatingWorld {
    fn new() -> Self;
    fn tick(&mut self, input: &[f64]) -> SimulationStatus;
    fn get_world_state(&self) -> Vec<f64>; // TODO: &[f64]
}
