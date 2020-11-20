
extern crate rand;

use rand::seq::SliceRandom;
use rand::Rng;

use crate::automata::Automata as Automata;
use crate::colour::Colour as Colour;

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

#[derive(Debug)]
pub struct Board {
    width: usize,
    height: usize,
    pub cell_states: Vec<Vec<usize>>,
    pub automata: Option<Automata>,
    pub apoptotic: bool,
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
