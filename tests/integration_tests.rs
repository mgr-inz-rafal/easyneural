extern crate easyneural;

use easyneural::trainer::Trainer;

#[test]
fn run_training_session() -> Result<(), String> {
    const POPULATION_SIZE: usize = 10;

    let neurons_per_layer = [2, 4, 5, 1];
    let mut trainer = Trainer::new(POPULATION_SIZE, &neurons_per_layer)?;

    let input1 = 0.0;
    let input2 = 1.1;

    let parents = trainer.run_session(&[input1, input2]);
    Ok(())
}
