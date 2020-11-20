extern crate rand;

use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Automata {
    pub rule_string: Vec<usize>,
    pub is_2d: bool,
    pub neighbourhood_size: u8,
    pub num_states: usize,
}

impl Automata {
    // num_states must be larger than 0, since having
    // a num_states value of 0 would cause an integer
    // overflow error when calculating the rule_string_length
    // i.e. -1 for a usize is not possible
    pub fn new1d(num_states: usize, neighbourhood_size: u8) -> Self {
        assert!(0 < num_states);
        let n_size = neighbourhood_size as usize;
        let rule_string_length = (num_states - 1) * (2 * n_size + 1) + 1;
        let rule_string = make_rules(rule_string_length, num_states);
        Automata {
            rule_string,
            is_2d: false,
            neighbourhood_size,
            num_states,
        }
    }

    pub fn new2d(num_states: usize, neighbourhood_size: u8) -> Self {
        assert!(0 < num_states);
        let n_width = neighbourhood_size as usize * 2 + 1;
        let rule_string_length = (num_states - 1) * n_width * n_width + 1;
        let rule_string = make_rules(rule_string_length, num_states);
        Automata {
            rule_string,
            is_2d: true,
            neighbourhood_size,
            num_states,
        }
    }
}

fn make_rules(length: usize, num_states: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut rule_string: Vec<usize> = (0..length).map(|_| rng.gen_range(0, num_states)).collect();
    // first rule must always be 0 since dead cells cannot
    // produce live cells
    if let Some(x) = rule_string.first_mut() {
        *x = 0;
    };
    rule_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "assertion failed: 0 < num_states")]
    fn empty1d() {
        let a1d = Automata::new1d(0, 1);
    }

    #[test]
    #[should_panic(expected = "assertion failed: 0 < num_states")]
    fn empty2d() {
        let a2d = Automata::new2d(0, 1);
    }

    #[test]
    fn make1d_all_dead() {
        let a1d = Automata::new1d(1, 1);
        assert_eq!(a1d.rule_string, vec![0]);
    }

    #[test]
    fn make1d_zero_neighbours() {
        let a1d = Automata::new1d(5, 0);
        assert_eq!(a1d.rule_string.len(), 5);
    }
}