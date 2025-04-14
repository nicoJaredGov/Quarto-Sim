use std::collections::HashSet;

use quarto_rust::quarto::Quarto;
use quarto_rust::quarto_agent::human_player::HumanPlayer;
use quarto_rust::quarto_agent::random_agent::RandomAgent;
use quarto_rust::quarto_agent::QuartoAgent;

fn main() {
    let random_agent1 = QuartoAgent::new(Box::new(RandomAgent {}));
    let random_agent2 = QuartoAgent::new(Box::new(RandomAgent {}));

    let mut quarto_game = Quarto::new(random_agent1, random_agent2);
    quarto_game.display_state();
    quarto_game.run();

    // let available_pieces: HashSet<u8> = vec![1, 6, 3, 15, 4].into_iter().collect();
    // for i in 0..5 {
    //     let ans = available_pieces.iter().next().unwrap().clone();
    //     println!("{i}: {ans}");
    // }
    
}
