use quarto_rust::quarto::Quarto;
use quarto_rust::quarto_agent::human_player::HumanPlayer;
use quarto_rust::quarto_agent::QuartoAgent;

fn main() {
    let player1 = HumanPlayer::new("Jared");
    let player1 = QuartoAgent::HumanPlayer(player1);

    let player2 = HumanPlayer::new("P2");
    let player2 = QuartoAgent::HumanPlayer(player2);

    let mut quarto_game = Quarto::new(player1, player2);
    quarto_game.display_board();
}
