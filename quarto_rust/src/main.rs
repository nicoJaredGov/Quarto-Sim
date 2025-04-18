use itertools::Itertools;
use std::collections::HashSet;

use quarto_rust::quarto::Quarto;
use quarto_rust::quarto_agent::QuartoAgent;
use quarto_rust::quarto_agent::human_player::HumanPlayer;
use quarto_rust::quarto_agent::negamax_agent::NegamaxAgent;
use quarto_rust::quarto_agent::random_agent::RandomAgent;
use quarto_rust::utils as qutils;

fn main() {
    let random_agent1 = QuartoAgent::new(Box::new(RandomAgent {}));
    let random_agent2 = QuartoAgent::new(Box::new(RandomAgent {}));
    let human = QuartoAgent::new(Box::new(HumanPlayer::new("Jared")));
    let human2 = QuartoAgent::new(Box::new(HumanPlayer::new("P2")));
    let negamax = QuartoAgent::new(Box::new(NegamaxAgent::new(3, 32)));

    let mut quarto_game: Quarto = Quarto::new(random_agent2, random_agent1);
    quarto_game.with_console_logs().with_file_logs();
    let result = quarto_game.run();
    quarto_game.reset();
    quarto_game.run();
    println!("{}", result);

    // let a: HashSet<i32> = vec![2, 3].into_iter().collect();
    // let b: HashSet<i32> = vec![1, 15, 16].into_iter().collect();
    // for r in a.iter().cartesian_product(b.iter()) {
    //     print!("{:?}", r);
    // }
    // println!("a {:?}\n b {:?}", a, b);

    // let l1 = [0, 1, 3];
    // let l2 = [4,9,11,5];
    // let l3: Vec<u8> = vec![4,9,11,5];
    // let d = qutils::matching_property_exists(&l1);
    // let e = qutils::matching_property_exists(&l3);
    // println!("a {:?}\n b {:?}", d, e);
}
