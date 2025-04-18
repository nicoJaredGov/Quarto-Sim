use std::fmt;

pub struct GameStats {
    pub p1_cumulative_time: u128,
    pub p2_cumulative_time: u128,
    pub p1_num_moves: u8,
    pub p2_num_moves: u8,
}

impl GameStats {
    pub fn new() -> Self {
        Self {
            p1_cumulative_time: 0,
            p2_cumulative_time: 0,
            p1_num_moves: 0,
            p2_num_moves: 0,
        }
    }

    pub fn reset(&mut self) {
        self.p1_cumulative_time = 0;
        self.p2_cumulative_time = 0;
        self.p1_num_moves = 0;
        self.p2_num_moves = 0;
    }
}

impl fmt::Display for GameStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = format!(
            "Game stats:\nPlayer 1 Time: {}\nPlayer 2 Time: {}\n",
            self.p1_cumulative_time, self.p2_cumulative_time
        );
        write!(f, "{}", message)
    }
}
