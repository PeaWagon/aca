use aca;

#[test]
fn example_2D() {

    /*
    num_mevs is the number of mating events (how many tournaments
    to run)
    population size is how many board to create
    num_states is how many states each position in the board
    can take on - by default it is 10 (0 through 9)
    the tournament size is how many to sample from the
    population (must be less than or equal to the population
    size)
    width and height are the size of 1D board (or size of 1
    2D layer)
    num_iters is how many boards to make for the 2D representation
    (if running 1D it should be 1)
    the start_population indicates:
      (for 1D) the first line of the board
      (for 2D) the centre square of the board
    max cuts indicates how many times a rule string
    can be cut before crossover operation is performed
    max mutations is how many point mutations can be
    performed on one rule string

    the output file will contain the evolved solutions
    to the apoptotic cellular automata problem

    */

    let num_mevs = 10;
    let population_size = 10;
    let num_states = 10;
    let tournament_size = 7;
    let neighbourhood_size = 1;
    let width = 51;
    let height = 51;
    let num_iters = 50;
    let start_population = [0,1,0,1,2,1,0,1,0];
    let max_cuts = 5;
    let max_mutations = 15;
    let output_file = "output.txt";

    let automata = aca::population::make_2d_population(
        population_size, num_states, neighbourhood_size
    );
    let mut boards = aca::board::make_boards(
        automata, width, height, num_iters, &start_population
    );

    for i in 0..num_mevs {
        println!("mev: {}", i);
        aca::tournament::tournament(tournament_size, &mut boards, max_cuts, max_mutations, &start_population, num_iters);
    }

    let results = aca::board::write_results(&boards, output_file);
    match results {
        Ok(()) => {
            println!("Output written to:\n{}", output_file);
        },
        Err(e) => {
            println!("Could not write results:\n{:?}", e);
        }
    }
}


#[test]
#[should_panic(expected = "Start population contains states that exceed limit for automata.")]
fn incorrect_num_states() {
    let a1d = aca::automata::Automata::new1d(5, 2);
    let mut board = aca::board::Board::initialise(100, 100);
    board.fill(a1d, &[5, 2, 1]);
}

#[test]
fn empty_board() {
    let a2d = aca::automata::Automata::new2d(5, 3);
    let mut board = aca::board::Board::initialise(100, 100);
    assert_eq!(board.cell_states, vec![vec![0; 100]; 100]);
    board.fill(a2d, &[0, 0, 0, 0]);
    assert_eq!(board.cell_states, vec![vec![0; 100]; 100]);
}

#[test]
fn make1d() {
    let a1d = aca::automata::Automata {
        is_2d: false,
        neighbourhood_size: 1,
        rule_string: vec![0, 1, 2, 2, 0, 0, 1],
        num_states: 3,
    };
    let mut board = aca::board::Board::initialise(5, 3);
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
    let a1d = aca::automata::Automata {
        is_2d: false,
        neighbourhood_size: 1,
        rule_string: vec![0, 0, 0, 0, 1, 0, 0],
        num_states: 3,
    };
    let mut board = aca::board::Board::initialise(5, 3);
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
    let a2d = aca::automata::Automata {
        is_2d: true,
        neighbourhood_size: 2,
        // 51 states
        rule_string: vec![
            0, 1, 1, 2, 2, 0, 0, 0, 1, 0, 2, 0, 1, 1, 2, 2, 0, 0, 0, 1, 2, 0, 0, 0, 1, 0, 0, 0,
            1, 0, 2, 0, 1, 1, 2, 2, 0, 0, 0, 1, 2, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0,
        ],
        num_states: 3,
    };
    let mut board = aca::board::Board::initialise(7, 7);
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
fn test_generic() {
    let a1d = aca::automata::Automata::new1d(3, 1);
    println!("{:?}", a1d);

    let mut board = aca::board::Board::initialise(5, 5);
    // println!("{:?}", board);

    board.fill(a1d, &[1, 2, 1]);
    board.next_board();
    println!("{:?}", board);

    let a2d = aca::automata::Automata::new2d(2, 1);
    println!("{:?}", a2d);

    board.empty();
    println!("{:?}", board);

    board.fill(a2d, &[1, 1, 1, 1]);
    println!("{:?}", board);
}
#[test]
fn run_tournament() {
    let mut boards: Vec<aca::board::Board> = Vec::with_capacity(10);
    let width = 100;
    let height = 100;
    let start_population = vec![1];
    for _ in 0..10 {
        let a1d = aca::automata::Automata::new1d(2, 1);
        let mut board = aca::board::Board::initialise(width, height);
        board.fill(a1d, &start_population);
        board.next_board();
        boards.push(board);
    }
    aca::tournament::tournament(4, &mut boards, 3, 3, &start_population, 1);
}

#[test]
fn run_mutate() {
    let num_states = 5;
    let mut a1d = aca::automata::Automata::new1d(num_states, 3);
    let copy_a1d = a1d.clone();
    let max_mutations = 10;
    aca::mutation::mutate(&mut a1d, max_mutations);
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
    let mut a1d = aca::automata::Automata::new1d(5, 3);
    let copy_a1d = a1d.clone();
    let max_mutations = 200;
    aca::mutation::mutate(&mut a1d, max_mutations);
}

#[test]
fn run_crossover() {
    let mut a1 = aca::automata::Automata::new1d(5, 2);
    let mut a2 = aca::automata::Automata::new1d(5, 2);
    println!("{:?}", a1.rule_string);
    println!("{:?}", a2.rule_string);
    aca::mutation::crossover(&mut a1, &mut a2, 5);
    println!("{:?}", a1.rule_string);
    println!("{:?}", a2.rule_string);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn mismatched_neighbourhood_sizes() {
    let mut a1 = aca::automata::Automata::new1d(5, 1);
    let mut a2 = aca::automata::Automata::new1d(5, 2);
    aca::mutation::crossover(&mut a1, &mut a2, 5);
}
#[test]
#[should_panic(expected = "assertion failed")]
fn mismatched_num_states() {
    let mut a1 = aca::automata::Automata::new1d(4, 2);
    let mut a2 = aca::automata::Automata::new1d(5, 2);
    aca::mutation::crossover(&mut a1, &mut a2, 5);
}
#[test]
#[should_panic(expected = "assertion failed")]
fn mismatched_automata_types() {
    let mut a1d = aca::automata::Automata::new1d(4, 2);
    let mut a2d = aca::automata::Automata::new2d(5, 2);
    aca::mutation::crossover(&mut a1d, &mut a2d, 5);
}

#[test]
fn see_result() {
    let mut b = aca::board::Board::initialise(100, 100);
    let a = aca::automata::Automata::new1d(10, 1);
    b.fill(a, &[1,2,1]);
    b.next_board();
    let result = b.result();
    println!("{}", result);
}
