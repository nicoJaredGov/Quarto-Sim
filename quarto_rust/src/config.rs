use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerConfig {
    pub agent_type: String,
    pub name: String,
    pub search_depth: u8,
    pub search_window: u8
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuartoSimConfig {
    pub player_one: PlayerConfig,
    pub player_two: PlayerConfig,
    pub num_runs: u16
}