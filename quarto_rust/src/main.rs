mod config;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use config::{PlayerConfig, QuartoSimConfig};
use quarto_rust::quarto::QuartoSimulator;
use quarto_rust::quarto_agent::QuartoAgent;
use quarto_rust::quarto_agent::human_player::HumanPlayer;
use quarto_rust::quarto_agent::negamax_agent::NegamaxAgent;
use quarto_rust::quarto_agent::random_agent::RandomAgent;

fn init_agent(player_config: &PlayerConfig) -> QuartoAgent {
    match player_config.agent_type.as_str() {
        "human" => QuartoAgent::new(Box::new(HumanPlayer::new(&player_config.name))),
        "random" => QuartoAgent::new(Box::new(RandomAgent {})),
        "negamax" => {
            let depth = player_config.search_depth;
            let window = player_config.search_window;
            QuartoAgent::new(Box::new(NegamaxAgent::new(depth, window)))
        }
        _ => QuartoAgent::new(Box::new(RandomAgent {})),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let use_config_arg = String::from("--use-config");
    let no_logs_arg = String::from("--no-logs");
    let player_one: QuartoAgent;
    let player_two: QuartoAgent;
    let game_config: QuartoSimConfig;

    if args.contains(&use_config_arg) {
        let mut file = File::open("config.json").expect("Error reading config file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read config file contents");
        //println!("file: {}", contents);
        game_config = serde_json::from_str(&contents).unwrap();
        //println!("config: {:?}", game_config);
    } else {
        game_config = QuartoSimConfig {
            player_one: PlayerConfig {
                agent_type: String::from("human"),
                name: String::from("human"),
                search_depth: 16,
                search_window: 16,
            },
            player_two: PlayerConfig {
                agent_type: String::from("human"),
                name: String::from("human"),
                search_depth: 16,
                search_window: 16,
            },
            num_runs: 10,
        }
    }

    player_one = init_agent(&game_config.player_one);
    player_two = init_agent(&game_config.player_two);
    let mut quarto_game: QuartoSimulator = QuartoSimulator::new(player_one, player_two);
    if !args.contains(&no_logs_arg) {
        quarto_game.with_file_logs();
    }
    quarto_game.run_multiple(game_config.num_runs);
}
