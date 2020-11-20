extern crate rand;

use rand::seq::SliceRandom;
use rand::Rng;

use crate::automata::Automata as Automata;

pub fn mutate(automata: &mut Automata, max_mutations: usize) {
    assert!(max_mutations <= automata.rule_string.len());
    let mut rng = rand::thread_rng();

    // choose a random number between 0 and max_mutations inclusive
    let num_mutations = rng.gen_range(0, max_mutations + 1);

    // pick where to perform the mutations (without replacement)
    let mut mutate_locations: Vec<usize> = (0..automata.rule_string.len()).collect();
    mutate_locations.shuffle(&mut rng);
    let mutate_locations = &mutate_locations[..num_mutations];

    // the only case in which the number of actual mutations is
    // less than num_mutations is when the random value chosen
    // is the same as the existing value in the rule string
    for location in mutate_locations {
        let random_value = rng.gen_range(0, automata.num_states);
        automata.rule_string[*location] = random_value;
    }
}

pub fn crossover(automata1: &mut Automata, automata2: &mut Automata, max_cuts: usize) {
    // make sure the automata are of the same type
    assert_eq!(automata1.is_2d, automata2.is_2d);
    assert_eq!(automata1.neighbourhood_size, automata2.neighbourhood_size);
    assert_eq!(automata1.num_states, automata2.num_states);
    assert!(max_cuts < automata1.rule_string.len());
    // how many points to perform crossover
    let mut rng = rand::thread_rng();
    let num_cuts = rng.gen_range(0, max_cuts + 1);

    let mut cut_locations: Vec<usize> = (0..automata1.rule_string.len()).collect();
    cut_locations.shuffle(&mut rng);

    // limit size of cut_locations so it matches num_cuts
    // sort the resulting vector
    // let mut cut_locations: Vec<usize> = &cut_locations[..num_cuts];
    cut_locations.split_off(num_cuts);
    cut_locations.sort();

    // iterate over elements in rule string, write to a different
    // vector each time a cut_location is encountered
    let mut rules1 = Vec::with_capacity(automata1.rule_string.len());
    let mut rules2 = Vec::with_capacity(automata1.rule_string.len());
    let mut swap = false;
    for i in 0..automata1.rule_string.len() {
        if cut_locations.contains(&i) {
            swap = !swap;
        }
        if swap {
            rules1.push(automata1.rule_string[i]);
            rules2.push(automata2.rule_string[i]);
        } else {
            rules1.push(automata2.rule_string[i]);
            rules2.push(automata1.rule_string[i]);
        }
    }
    automata1.rule_string = rules1;
    automata2.rule_string = rules2;
}