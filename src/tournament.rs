extern crate rand;

use rand::seq::SliceRandom;
use rand::Rng;

use crate::board::Board as Board;
use crate::mutation::{crossover, mutate};

pub fn tournament(
    tournament_size: usize,
    population: &mut Vec<Board>,
    max_cuts: usize,
    max_mutations: usize,
    start_population: &[usize],
    num_iters: usize,
) {
    assert!(tournament_size <= population.len());

    let mut rng = rand::thread_rng();

    let mut sample: Vec<usize> = (0..population.len()).collect();
    sample.shuffle(&mut rng);

    // limit size of sample so it matches population size
    let sample = &sample[..tournament_size];

    let mut fitness_values: Vec<usize> = Vec::with_capacity(tournament_size);

    for i in 0..tournament_size {
        fitness_values.push(population[sample[i]].fitness);
    }
    let (max_idx1, max_idx2) = max_two_indices(&fitness_values);
    let mut child1 = population[sample[max_idx1]]
        .automata
        .as_ref()
        .expect("Board is missing automata.")
        .clone();
    let mut child2 = population[sample[max_idx2]]
        .automata
        .as_ref()
        .expect("Board is missing automata.")
        .clone();

    // crossover and then mutate
    crossover(&mut child1, &mut child2, max_cuts);
    mutate(&mut child1, max_mutations);
    mutate(&mut child2, max_mutations);

    // replace worst 2 with mutated parents
    // in the case where the population size is less than 4,
    // then the children should replace the parents
    let (min_idx1, min_idx2) = min_two_indices(&fitness_values);
    population[sample[min_idx1]].empty();
    population[sample[min_idx1]].run(child1, start_population, num_iters);
    population[sample[min_idx2]].empty();
    population[sample[min_idx2]].run(child2, start_population, num_iters);
}

fn max_two_indices(s: &Vec<usize>) -> (usize, usize) {
    // given a borrowed vector, pick the indices where
    // the largest two values reside and return those
    // indices as a tuple (max1, max2), where
    // s[max2] < s[max1]
    assert!(1 < s.len());
    let mut max1: usize;
    let mut max2: usize;
    if s[0] < s[1] {
        max1 = 1;
        max2 = 0;
    } else {
        max1 = 0;
        max2 = 1;
    }
    for i in 2..s.len() {
        if s[max1] < s[i] {
            max2 = max1;
            max1 = i;
        } else if s[max2] < s[i] {
            max2 = i;
        }
    }
    (max1, max2)
}

fn min_two_indices(s: &Vec<usize>) -> (usize, usize) {
    // given a borrowed vector, pick the indices where
    // the smallest two values reside and return those
    // indices as a tuple (min1, min2), where
    // s[min1] < s[min2]
    assert!(1 < s.len());
    let mut min1: usize;
    let mut min2: usize;
    if s[s.len() - 2] < s[s.len() - 1] {
        min1 = s.len() - 2;
        min2 = s.len() - 1;
    } else {
        min1 = s.len() - 1;
        min2 = s.len() - 2;
    }
    // reverse here is so it works well with max function
    // i.e. if all fitness values are the same, then min
    // will return last 2 indices whereas max will return
    // first two indices
    for i in (0..s.len() - 2).rev() {
        if s[i] < s[min1] {
            min2 = min1;
            min1 = i;
        } else if s[i] < s[min2] {
            min2 = i;
        }
    }
    (min1, min2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum() {
        let mut result: (usize, usize);
        let v0 = vec![0, 4, 2, 5, 3];
        result = max_two_indices(&v0);
        assert_eq!(result, (3, 1));

        let v1 = vec![0, 0, 0, 0, 0];
        result = max_two_indices(&v1);
        assert_eq!(result, (0, 1));

        let v2 = vec![0, 0, 0, 0, 1];
        result = max_two_indices(&v2);
        assert_eq!(result, (4, 0));

        let v3 = vec![0, 0, 435, 22, 7];
        result = max_two_indices(&v3);
        assert_eq!(result, (2, 3));

        let v4 = vec![5454, 33, 435, 22, 7];
        result = max_two_indices(&v4);
        assert_eq!(result, (0, 2));
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn maximum_2_of_1() {
        let v0 = vec![0];
        let result = max_two_indices(&v0);
        assert!(false, result);
    }
    #[test]
    fn test_minimum() {
        let mut result: (usize, usize);
        let v0 = vec![45, 232, 654, 23, 54, 2, 0];
        result = min_two_indices(&v0);
        assert_eq!(result, (6, 5));

        let v1 = vec![0, 0, 0];
        result = min_two_indices(&v1);
        assert_eq!(result, (2, 1));
    }
}