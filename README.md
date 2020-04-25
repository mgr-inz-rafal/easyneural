# Easyneural

[![Build Status](https://travis-ci.com/mgr-inz-rafal/easyneural.svg?token=1BHPtQwo7AzbGygkvjYy&branch=master)](https://travis-ci.com/mgr-inz-rafal/easyneural)

# Documentation

This crate let's you build and train the neural network **easily**.

For now, just a quick example...

```Rust
fn main() {
    const POPULATION_SIZE: usize = 10;
    const SIMULATION_ROUNDS: usize = 1;

    let neurons_per_layer = [2, 4, 5, 1];
    let mut randomizer = DefaultRandomizer::new();

    if_chain! {
        if let Ok(mut session) = Simulation::<MyWorld>::new(POPULATION_SIZE, &neurons_per_layer, &mut randomizer, None);
        if let Ok(parents) = session.run(Finish::Occurences(SIMULATION_ROUNDS));
        then {
            // I have the trained network!
        }
    }
}
```


...and a movie of a car that learned on its own how to avoid cows :)

https://www.youtube.com/watch?v=pjrmog-Sp6w