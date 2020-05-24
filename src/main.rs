use aca::*;

fn main() {

    let num_mevs = 10000;
    let population_size = 100;
    let num_states = 10;
    let tournament_size = 7;
    let neighbourhood_size = 1;
    let width = 201;
    let height = 201;
    let num_iters = 50;
    let start_population = [0,1,0,1,2,1,0,1,0];
    let max_cuts = 5;
    let max_mutations = 15;
    let output_file = "output.txt";

    let automata = make_2d_population(
        population_size, num_states, neighbourhood_size
    );
    let mut boards = make_boards(
        automata, width, height, num_iters, &start_population
    );

    for i in 0..num_mevs {
        println!("mev: {}", i);
        tournament(tournament_size, &mut boards, max_cuts, max_mutations, &start_population, num_iters);
    }

    let results = write_results(&boards, output_file);
    match results {
        Ok(()) => {
            println!("Output written to:\n{}", output_file);
        },
        Err(e) => {
            println!("Could not write results:\n{:?}", e);
        }
    }
}
