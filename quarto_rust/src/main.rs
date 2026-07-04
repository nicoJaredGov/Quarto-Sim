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

pub const USE_CONFIG_ARG: &'static str = "--use-config";
pub const NO_LOGS_ARG: &'static str = "--no-logs";

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

fn default_game_config() -> QuartoSimConfig {
    QuartoSimConfig {
        player_one: PlayerConfig {
            agent_type: String::from("random"),
            name: String::from("human"),
            search_depth: 16,
            search_window: 16,
        },
        player_two: PlayerConfig {
            agent_type: String::from("radom"),
            name: String::from("human"),
            search_depth: 16,
            search_window: 16,
        },
        num_runs: 10,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let game_config = match args.iter().any(|a| a == USE_CONFIG_ARG) {
        true => {
            let mut file = File::open("config.json").expect("Error reading config.json file");
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Unable to read config file contents");
            serde_json::from_str(&contents).expect("Error parsing json config file")
        }
        false => default_game_config(),
    };

    let player_one = init_agent(&game_config.player_one);
    let player_two = init_agent(&game_config.player_two);
    let mut quarto_game: QuartoSimulator = QuartoSimulator::new(player_one, player_two);

    if !args.iter().any(|a| a == NO_LOGS_ARG) {
        quarto_game.with_file_logs();
    }
    quarto_game.run_multiple(game_config.num_runs);
}
