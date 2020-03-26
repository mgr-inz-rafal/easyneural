extern crate easyneural;

use easyneural::trainer::Trainer;

#[test]
fn run_training_session() {
    const POPULATION_SIZE: usize = 10;

    let neurons_per_layer = [2, 4, 5, 1];
    let trainer = Trainer::new(POPULATION_SIZE, &neurons_per_layer);

    let input1 = 0.0;
    let input2 = 1.1;

    trainer.run_session(&[input1, input2]);
}
