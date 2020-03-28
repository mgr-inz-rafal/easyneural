use crate::trainer::Specimen;

pub trait SimulatingWorld {
    fn new() -> Self;
    fn release_specimen(&mut self, specimen: &mut Specimen);
}
