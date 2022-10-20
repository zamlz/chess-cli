use std::io;

use crate::chess::error::SystemError;
use crate::chess::piece;
use crate::utils::ColorString;

pub enum GameState {
    InProgress,
    Check,
    Checkmate,
    Stalemate,
}

pub struct ChessBoard {
    map: [[Option<Box<dyn piece::ChessPiece>>; 8]; 8],
}

impl ChessBoard {
    pub fn new() -> ChessBoard {
        let mut chess_board = ChessBoard {
            map: Default::default(),
        };
        for i in 0..8 {
            chess_board.map[1][i] = Some(Box::new(piece::Pawn {
                color: piece::PieceColor::White,
            }));
            chess_board.map[6][i] = Some(Box::new(piece::Pawn {
                color: piece::PieceColor::Black,
            }));
        }
        for i in [0, 7] {
            let piece_color = if i == 0 {
                piece::PieceColor::White
            } else {
                piece::PieceColor::Black
            };
            chess_board.map[i][0] = Some(Box::new(piece::Rook { color: piece_color }));
            chess_board.map[i][1] = Some(Box::new(piece::Knight { color: piece_color }));
            chess_board.map[i][2] = Some(Box::new(piece::Bishop { color: piece_color }));
            chess_board.map[i][3] = Some(Box::new(piece::Queen { color: piece_color }));
            chess_board.map[i][4] = Some(Box::new(piece::King { color: piece_color }));
            chess_board.map[i][5] = Some(Box::new(piece::Bishop { color: piece_color }));
            chess_board.map[i][6] = Some(Box::new(piece::Knight { color: piece_color }));
            chess_board.map[i][7] = Some(Box::new(piece::Rook { color: piece_color }));
        }
        return chess_board;
    }

    pub fn display(&self, player: &piece::PieceColor) {
        println!("\n\n   *---+---+---+---+---+---+---+---*");
        for i in 0..8 {
            // compute the real rank
            let rank = if let piece::PieceColor::White = player {
                7 - i
            } else {
                i
            };
            print!(" {} |", rank + 1);

            for j in 0..8 {
                // Compute the real file
                let file = if let piece::PieceColor::White = player {
                    j
                } else {
                    7 - j
                };
                // color the square
                if (rank + file) % 2 == 0 {
                    print!("{} ", ColorString::BgBlack.as_str());
                } else {
                    print!("{} ", ColorString::BgWhite.as_str());
                }

                // color piece if it exists
                match &self.map[rank][file] {
                    Some(piece) => {
                        print!("{}", piece.get_color());
                        print!("{}", piece.get_icon());
                    }
                    None => {
                        print!(" ");
                    }
                }
                print!(" {}|", ColorString::Reset.as_str());
            }
            println!("");
            println!("   *---+---+---+---+---+---+---+---*");
        }

        // print the file index
        if let piece::PieceColor::White = player {
            println!("     a   b   c   d   e   f   g   h  ");
        } else {
            println!("     h   g   f   e   d   c   b   a  ");
        }
    }

    fn get_action(&self) -> Result<[piece::Position; 2], SystemError> {
        println!("\nEnter Action: ");
        let mut action = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read input");
        println!("Entered action is {}", action);
        let action_vec: Vec<char> = action.chars().collect();
        if action.len() == 6 && action_vec[2] == ' ' {
            let start = piece::Position::new([action_vec[0], action_vec[1]]);
            let end = piece::Position::new([action_vec[3], action_vec[4]]);
            match (start, end) {
                (Result::Err(start_error), Result::Err(end_error)) => Result::Err(start_error),
                (Result::Err(error), _) => Result::Err(error),
                (_, Result::Err(error)) => Result::Err(error),
                (Result::Ok(start_pos), Result::Ok(end_pos)) => Result::Ok([start_pos, end_pos]),
            }
        } else {
            Result::Err(SystemError::BadActionString(action))
        }
    }

    fn execute_action(
        &self,
        start: piece::Position,
        end: piece::Position,
    ) -> Result<GameState, SystemError> {
        let board_square = &self.map[start.rank as usize][start.file as usize];
        match board_square {
            Some(piece) => match piece.validate_move(start, end) {
                true => Result::Ok(GameState::InProgress),
                false => Result::Err(SystemError::InvalidPieceMove),
            },
            None => Result::Err(SystemError::PieceMissing),
        }
    }

    pub fn take_turn(&self, player: &piece::PieceColor) -> Result<GameState, SystemError> {
        let action = self.get_action();
        match action {
            Result::Ok(position_list) => self.execute_action(position_list[0], position_list[1]),
            Result::Err(error) => Result::Err(error),
        }
    }
}
