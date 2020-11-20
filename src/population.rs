
use crate::automata::Automata as Automata;

pub fn make_1d_population(size: usize, num_states: usize, neighbourhood_size: u8) -> Vec<Automata> {
    let mut population = Vec::with_capacity(size);
    for _ in 0..size {
        let a = Automata::new1d(num_states, neighbourhood_size);
        population.push(a);
    }
    population
}

pub fn make_2d_population(size: usize, num_states: usize, neighbourhood_size: u8) -> Vec<Automata> {
    let mut population = Vec::with_capacity(size);
    for _ in 0..size {
        let a = Automata::new2d(num_states, neighbourhood_size);
        population.push(a);
    }
    population
}