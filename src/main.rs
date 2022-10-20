/*    ________                      ________    ____
 *   / ____/ /_  ___  __________   / ____/ /   /  _/
 *  / /   / __ \/ _ \/ ___/ ___/  / /   / /    / /
 * / /___/ / / /  __(__  |__  )  / /___/ /____/ /
 * \____/_/ /_/\___/____/____/   \____/_____/___/
 *                          - Amlesh Sivanantham
 */

use crate::chess::board::ChessBoard;
use crate::chess::board::GameState;
use crate::chess::piece;
use crate::utils::ColorString;

mod chess;
mod utils;

// --------------------------------------------------------------------------
// MAIN GAME LOOP
// --------------------------------------------------------------------------

fn main() {
    print!("{}", ColorString::FgBlue.as_str());
    println!("  ___ _                 ___ _    ___ ");
    println!(" / __| |_  ___ ______  / __| |  |_ _|");
    println!("| (__| ' \\/ -_|_-<_-< | (__| |__ | | ");
    println!(" \\___|_||_\\___/__/__/  \\___|____|___|");
    print!("{}", ColorString::FgRed.as_str());
    println!("                              v0.0.1");
    print!("{}", ColorString::Reset.as_str());

    let chess_board = ChessBoard::new();
    let mut player_turn = piece::PieceColor::White;

    loop {
        // Primary Game Loop Logic
        chess_board.display(&player_turn);
        match chess_board.take_turn(&player_turn) {
            Result::Ok(game_state) => {
                player_turn = match player_turn {
                    piece::PieceColor::White => piece::PieceColor::Black,
                    piece::PieceColor::Black => piece::PieceColor::White,
                };
                match game_state {
                    GameState::Stalemate => {
                        println!("The game has been stalemated!");
                        break;
                    }
                    GameState::Checkmate => {
                        println!("You have been Checkmated!");
                        break;
                    }
                    GameState::Check => {
                        println!("You are in Check!");
                    }
                    GameState::InProgress => {}
                }
            }
            Result::Err(error) => {
                println!("Encountered Error: {:?}", error);
            }
        }
    }
}
