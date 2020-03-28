use crate::trainer::Specimen;

// TODO: Should be named differently, I think...
pub trait World {
    fn new() -> Self;
    fn release_specimen(&mut self, specimen: &mut Specimen);
}
