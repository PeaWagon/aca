extern crate rand;

use rand::seq::SliceRandom;
use rand::Rng;

pub fn write_results(boards: &Vec<Board>, output_file: &str) -> std::io::Result<()> {

    let mut results = String::new();
    for board in boards {
        let result = board.result();
        results.push_str(&result);
        results.push_str("\n");
    }

    std::fs::write(output_file, results)?;
    Ok(())
}

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

pub fn make_boards(mut population: Vec<Automata>, width: usize, height: usize, num_iters: usize, start_population: &[usize]) -> Vec<Board> {
    let mut boards = Vec::with_capacity(population.len());
    for _ in 0..population.len() {
        let mut board = Board::initialise(width, height);
        let automata = population.pop().unwrap();
        board.fill(automata, &start_population);
        for _ in 0..num_iters {
            board.next_board();
            if !board.apoptotic {
                break;
            }
        }
        board.is_apoptotic();
        boards.push(board);
    }
    boards
}

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

fn mutate(automata: &mut Automata, max_mutations: usize) {
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

fn crossover(automata1: &mut Automata, automata2: &mut Automata, max_cuts: usize) {
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

#[derive(Debug)]
pub struct Board {
    width: usize,
    height: usize,
    cell_states: Vec<Vec<usize>>,
    automata: Option<Automata>,
    apoptotic: bool,
    pub fitness: usize,
}

impl Board {

    pub fn initialise(width: usize, height: usize) -> Board {
        // board defaults to being apoptotic until the wall is
        // hit or the last iteration has alive cells
        Board {
            width,
            height,
            cell_states: vec![vec![0; width]; height],
            automata: None,
            apoptotic: true,
            fitness: 0,
        }
    }

    pub fn run(&mut self, automata: Automata, start_population: &[usize], num_iters: usize) {
        self.fill(automata, start_population);
        self.next_board();
        for _ in 0..num_iters - 1 {
            self.next_board();
            if !self.apoptotic {
                break;
            }
        }
        self.is_apoptotic();
    }

    pub fn fill(&mut self, automata: Automata, start_population: &[usize]) {
        // the starting population cannot be wider
        // than there is space available on the board
        // (or taller in the 2d case)
        for i in start_population {
            if automata.num_states <= *i {
                panic!("Start population contains states that exceed limit for automata.");
            }
        }

        // integer overflow (negative value for an unsigned integer)
        // can be caused if these checks do not pass
        assert!((automata.neighbourhood_size as usize + 1) / 2 <= self.height);
        assert!((automata.neighbourhood_size as usize + 1) / 2 <= self.width);

        self.automata = Some(automata);

        if self.automata.as_ref().unwrap().is_2d {
            let start_size = start_population.len();
            let start_width = (start_size as f64).sqrt();
            if start_width % 1.0 != 0.0 {
                panic!("Length of start population should be a perfect square (i.e. can be square rooted without decimal).");
            }
            let start_width = start_width as usize;
            assert!(start_width <= self.width);
            assert!(start_width <= self.height);
            let padding_top = (self.height - start_width) / 2;
            let padding_left = (self.width - start_width) / 2;

            for i in padding_top..padding_top + start_width {
                for j in padding_left..padding_left + start_width {
                    let new_value =
                        start_population[(i - padding_top) * start_width + (j - padding_left)];
                    self.cell_states[i][j] = new_value;
                    if new_value != 0 {
                        self.fitness += 1;
                    }
                }
            }
            // check if current layer hits wall
            for c in &self.cell_states[0] {
                if *c != 0 {
                    self.apoptotic = false;
                    return;
                }
            }
            for c in &self.cell_states[self.height - 1] {
                if *c != 0 {
                    self.apoptotic = false;
                    return;
                }
            }
            for row_num in 1..self.height - 1 {
                if self.cell_states[row_num][0] != 0 || self.cell_states[row_num][self.width - 1] != 0 {
                    self.apoptotic = false;
                    return;
                }
            }
        } else {
            // make sure initial population can fit inside allocated width
            let start_width = start_population.len();
            assert!(start_width <= self.width);
            // determine how much padding to add if initial population
            // is centred; excess padding will go to the right end
            let padding = (self.width - start_width) / 2;
            // fill in start population
            for i in padding..padding + start_width {
                let new_value = start_population[i - padding];
                self.cell_states[0][i] = new_value;
                if new_value != 0 {
                    self.fitness += 1;
                }
            }
            // see if first row already hit wall
            if self.cell_states[0][0] != 0 || self.cell_states[0][self.width - 1] != 0 {
                self.apoptotic = false;
            }
        }
    }

    pub fn next_board(&mut self) {
        // given the current state of the board, use
        // the rule string to create the next layer
        // account for neighbourhood size
        let automata: &Automata;
        if self.automata.is_some() {
            automata = self.automata.as_ref().unwrap();
        } else {
            panic!("No automata in the input board.");
        }

        if self.automata.as_ref().unwrap().is_2d {
            let prev_board = self.cell_states.clone();
            for j in 0..self.height {
                let start_row =
                    ((j + self.height) - automata.neighbourhood_size as usize) % self.height;
                for k in 0..self.width {
                    let start_column =
                        ((k + self.width) - automata.neighbourhood_size as usize) % self.width;
                    let mut neighbourhood_sum = 0;
                    let mut current_row = start_row;
                    let mut current_column = start_column;
                    for _ in 0..automata.neighbourhood_size * 2 + 1 {
                        if current_row == self.height {
                            current_row = 0;
                        }
                        for _ in 0..automata.neighbourhood_size * 2 + 1 {
                            if current_column == self.width {
                                current_column = 0;
                            }
                            neighbourhood_sum += prev_board[current_row][current_column];
                            current_column += 1;
                        }
                        current_row += 1;
                        current_column = start_column;
                    }
                    let new_state = automata.rule_string[neighbourhood_sum];
                    if new_state != 0 {
                        self.fitness += 1;
                    }
                    self.cell_states[j][k] = new_state;
                }
            }
            // check if current layer hits wall
            for c in &self.cell_states[0] {
                if *c != 0 {
                    self.apoptotic = false;
                    return;
                }
            }
            for c in &self.cell_states[self.height - 1] {
                if *c != 0 {
                    self.apoptotic = false;
                    return;
                }
            }
            for row_num in 1..self.height - 1 {
                if self.cell_states[row_num][0] != 0 || self.cell_states[row_num][self.width - 1] != 0 {
                    self.apoptotic = false;
                    return;
                }
            }
        } else {
            for i in 1..self.height {
                for j in 0..self.width {
                    let left_index =
                        ((j + self.width) - automata.neighbourhood_size as usize) % self.width;
                    let mut neighbourhood_sum = 0;
                    let mut current_index = left_index;
                    // since the neighbourhood size is variable, have to account for
                    // cases where the neighbourhood wraps
                    for _ in 0..automata.neighbourhood_size * 2 + 1 {
                        if current_index == self.width {
                            current_index = 0;
                        }
                        neighbourhood_sum += self.cell_states[i - 1][current_index];
                        current_index += 1;
                    }
                    let new_state = automata.rule_string[neighbourhood_sum];
                    if new_state != 0 {
                        self.fitness += 1;
                    }
                    self.cell_states[i][j] = new_state;
                }
                // check if wall was hit
                if self.cell_states[i][0] != 0 || self.cell_states[i][self.width - 1] != 0 {
                    self.apoptotic = false;
                }
            }
        }
    }

    pub fn is_apoptotic(&mut self) -> bool {
        // this function is called after the completion
        // of the board to see if the automata was
        // apoptotic
        if self.automata.is_none() {
            panic!("Missing automata");
        }
        if !self.apoptotic {
            // the automata might already be apoptotic
            // since it hit the wall during growth
            self.fitness = 0;
            return false;
        }
        if self.automata.as_ref().unwrap().is_2d {
            // check if the current board is empty
            for i in 0..self.height {
                for j in 0..self.width {
                    if self.cell_states[i][j] != 0 {
                        self.apoptotic = false;
                        self.fitness = 0;
                        return false;
                    }
                }
            }
        } else {
            // check if all last row is dead
            for c in &self.cell_states[self.height - 1] {
                if *c != 0 {
                    self.apoptotic = false;
                    self.fitness = 0;
                    return false;
                }
            }
        }
        true
    }

    fn generate_colours(&self) -> Vec<Colour> {
        if self.automata.is_none() {
            panic!("Missing automata");
        }
        // make random colours to match
        // the number of states
        let mut rng = rand::thread_rng();
        let mut colours = Vec::with_capacity(self.automata.as_ref().unwrap().num_states);
        for _ in 0..self.automata.as_ref().unwrap().num_states {
            let r: u8 = rng.gen();
            let g: u8 = rng.gen();
            let b: u8 = rng.gen();
            let a: f64 = rng.gen();
            let c = Colour::new(r, g, b, a);
            colours.push(c);
        }
        colours
    }

    pub fn as_html_table(&self, colours: &Vec<Colour>) -> String {
        // make a html table based on the current state
        // note <table></table> already in html
        let mut contents = "".to_string();
        for row in &self.cell_states {
            contents.push_str("<tr>");
            for col in row {
                contents.push_str("<td class='cell' style='background-color: ");
                let c = colours.get(*col).expect("Not enough colours for all cell states");
                contents.push_str(
                    &format!("rgba({}, {}, {}, {});'></td>", c.r, c.g, c.b, c.a)
                );
            }
            contents.push_str("</tr>");
        }
        contents
    }

    pub fn empty(&mut self) {
        // delete the current cell states, automata, and fitness
        // reset the apoptotic flag
        self.cell_states = vec![vec![0; self.width]; self.height];
        self.automata = None;
        self.apoptotic = true;
        self.fitness = 0;
    }

    pub fn result(&self) -> String {
        let mut result_string = String::new();
        if let Some(x) = &self.automata {
            for c in &x.rule_string {
                let value = c.to_string();
                result_string.push_str(&value);
            }
            result_string.push(',');
            result_string.push(' ');
            result_string.push_str(&self.fitness.to_string());
        } else {
            result_string = "None".to_string();
        }
        result_string
    }
}



#[derive(Debug)]
pub struct Colour {
    // red, green, blue, opacity
    r: u8,
    g: u8,
    b: u8,
    a: f64,
}

impl Colour {
    pub fn new(r: u8, g: u8, b: u8, a: f64) -> Colour {
        assert!(0.0 <= a && a <= 1.0);
        Colour { r, g, b, a }
    }
}

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

    #[test]
    #[should_panic(expected = "Start population contains states that exceed limit for automata.")]
    fn incorrect_num_states() {
        let a1d = Automata::new1d(5, 2);
        let mut board = Board::initialise(100, 100);
        board.fill(a1d, &[5, 2, 1]);
    }

    #[test]
    fn empty_board() {
        let a2d = Automata::new2d(5, 3);
        let mut board = Board::initialise(100, 100);
        assert_eq!(board.cell_states, vec![vec![0; 100]; 100]);
        board.fill(a2d, &[0, 0, 0, 0]);
        assert_eq!(board.cell_states, vec![vec![0; 100]; 100]);
    }

    #[test]
    fn make1d() {
        let a1d = Automata {
            is_2d: false,
            neighbourhood_size: 1,
            rule_string: vec![0, 1, 2, 2, 0, 0, 1],
            num_states: 3,
        };
        let mut board = Board::initialise(5, 3);
        board.fill(a1d, &[1, 2, 1]);
        assert_eq!(
            board.cell_states,
            vec![
                vec![0, 1, 2, 1, 0],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0],
            ]
        );
        board.next_board();
        assert_eq!(
            board.cell_states,
            vec![
                vec![0, 1, 2, 1, 0],
                vec![1, 2, 0, 2, 1],
                vec![0, 2, 0, 2, 0],
            ]
        );
        board.is_apoptotic();
        assert_eq!(board.fitness, 0);
        assert_eq!(board.apoptotic, false);
    }

    #[test]
    fn make1d_apoptotic() {
        let a1d = Automata {
            is_2d: false,
            neighbourhood_size: 1,
            rule_string: vec![0, 0, 0, 0, 1, 0, 0],
            num_states: 3,
        };
        let mut board = Board::initialise(5, 3);
        board.fill(a1d, &[1, 2, 1]);
        board.next_board();
        assert_eq!(
            board.cell_states,
            vec![
                vec![0, 1, 2, 1, 0],
                vec![0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0],
            ]
        );
        assert_eq!(board.fitness, 4);
        assert_eq!(board.apoptotic, true);
    }

    #[test]
    fn make_2d() {
        let a2d = Automata {
            is_2d: true,
            neighbourhood_size: 2,
            // 51 states
            rule_string: vec![
                0, 1, 1, 2, 2, 0, 0, 0, 1, 0, 2, 0, 1, 1, 2, 2, 0, 0, 0, 1, 2, 0, 0, 0, 1, 0, 0, 0,
                1, 0, 2, 0, 1, 1, 2, 2, 0, 0, 0, 1, 2, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0,
            ],
            num_states: 3,
        };
        let mut board = Board::initialise(7, 7);
        board.fill(a2d, &[1, 2, 0, 1]);
        assert_eq!(
            board.cell_states,
            vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 2, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ]
        );
        board.next_board();
        assert_eq!(
            board.cell_states,
            vec![
                vec![1, 2, 2, 2, 2, 1, 0],
                vec![1, 2, 2, 2, 2, 2, 0],
                vec![1, 2, 2, 2, 2, 2, 0],
                vec![1, 2, 2, 2, 2, 2, 0],
                vec![1, 2, 2, 2, 2, 2, 0],
                vec![0, 1, 1, 1, 1, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
            ]
        );
        assert_eq!(board.apoptotic, false);
        board.is_apoptotic();
        assert_eq!(board.fitness, 0);
    }

    #[test]
    fn make_colour() {
        let white = Colour::new(0, 0, 0, 1.0);
        let black = Colour::new(255, 255, 255, 1.0);
        assert_eq!(white.a, black.a);
        assert_ne!(white.r, black.r);
        assert_eq!(black.g, 255);
        let red = Colour::new(255, 0, 0, 1.0);
        let green = Colour::new(0, 255, 0, 1.0);
        let blue = Colour::new(0, 0, 255, 1.0);
        println!("{:?}", red);
        println!("{:?}", green);
        println!("{:?}", blue);
    }

    #[test]
    fn test_generic() {
        let a1d = Automata::new1d(3, 1);
        println!("{:?}", a1d);

        let mut board = Board::initialise(5, 5);
        // println!("{:?}", board);

        board.fill(a1d, &[1, 2, 1]);
        board.next_board();
        println!("{:?}", board);

        let a2d = Automata::new2d(2, 1);
        println!("{:?}", a2d);

        board.empty();
        println!("{:?}", board);

        board.fill(a2d, &[1, 1, 1, 1]);
        println!("{:?}", board);
    }
    #[test]
    fn run_tournament() {
        let mut boards: Vec<Board> = Vec::with_capacity(10);
        let width = 100;
        let height = 100;
        let start_population = vec![1];
        for _ in 0..10 {
            let a1d = Automata::new1d(2, 1);
            let mut board = Board::initialise(width, height);
            board.fill(a1d, &start_population);
            board.next_board();
            boards.push(board);
        }
        tournament(4, &mut boards, 3, 3, &start_population, 1);
    }

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

    #[test]
    fn run_mutate() {
        let num_states = 5;
        let mut a1d = Automata::new1d(num_states, 3);
        let copy_a1d = a1d.clone();
        let max_mutations = 10;
        mutate(&mut a1d, max_mutations);
        let mut num_changes = 0;
        for (i, j) in a1d.rule_string.iter().zip(copy_a1d.rule_string.iter()) {
            if i != j {
                num_changes += 1;
            }
            // new value for mutation should not exceed num_states - 1
            assert!(*i < num_states);
            assert!(*j < num_states);
        }
        assert!(num_changes <= max_mutations);
        println!("{}", num_changes);
        println!("{:?}", copy_a1d.rule_string);
        println!("{:?}", a1d.rule_string);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn too_many_mutations() {
        let mut a1d = Automata::new1d(5, 3);
        let copy_a1d = a1d.clone();
        let max_mutations = 200;
        mutate(&mut a1d, max_mutations);
    }

    #[test]
    fn run_crossover() {
        let mut a1 = Automata::new1d(5, 2);
        let mut a2 = Automata::new1d(5, 2);
        println!("{:?}", a1.rule_string);
        println!("{:?}", a2.rule_string);
        crossover(&mut a1, &mut a2, 5);
        println!("{:?}", a1.rule_string);
        println!("{:?}", a2.rule_string);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn mismatched_neighbourhood_sizes() {
        let mut a1 = Automata::new1d(5, 1);
        let mut a2 = Automata::new1d(5, 2);
        crossover(&mut a1, &mut a2, 5);
    }
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn mismatched_num_states() {
        let mut a1 = Automata::new1d(4, 2);
        let mut a2 = Automata::new1d(5, 2);
        crossover(&mut a1, &mut a2, 5);
    }
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn mismatched_automata_types() {
        let mut a1d = Automata::new1d(4, 2);
        let mut a2d = Automata::new2d(5, 2);
        crossover(&mut a1d, &mut a2d, 5);
    }

    #[test]
    fn see_result() {
        let mut b = Board::initialise(100, 100);
        let a = Automata::new1d(10, 1);
        b.fill(a, &[1,2,1]);
        b.next_board();
        let result = b.result();
        println!("{}", result);
    }
}
