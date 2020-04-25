use crate::network::Network;

/// Status of a specimen.
pub enum SpecimenStatus {
    /// Specimen is still alive with the specified fitness.
    ALIVE(f64),
    /// Specimen is dead with the specified fitness. Dead specimen marks the end of
    /// the single learning loop.
    DEAD(f64),
}

#[derive(Clone)]
pub(crate) struct Specimen {
    pub brain: Network,
    pub(crate) fitness: f64, // TODO: Move to SpecimenStatus
}

impl Specimen {
    pub(crate) fn tick(&mut self, world_input: &[f64]) -> Vec<f64> {
        self.brain.fire(world_input);
        self.brain.get_output()
    }
}
