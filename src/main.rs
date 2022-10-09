/*    ________                      ________    ____
 *   / ____/ /_  ___  __________   / ____/ /   /  _/
 *  / /   / __ \/ _ \/ ___/ ___/  / /   / /    / /
 * / /___/ / / /  __(__  |__  )  / /___/ /____/ /
 * \____/_/ /_/\___/____/____/   \____/_____/___/
 */

use std::io;

// --------------------------------------------------------------------------
// Enum Models
// --------------------------------------------------------------------------

enum PieceColor {
    Black,
    White,
    Empty,
}

enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
    EmptySpace,
}

// --------------------------------------------------------------------------
// CHESS PIECE
// --------------------------------------------------------------------------

struct Piece {
    color: PieceColor,
    ptype: PieceType,
}

impl Piece {
    fn new(ascii: char, piece_color: PieceColor) -> Piece {
        match ascii {
            'K' => Piece {
                ptype: PieceType::King,
                color: piece_color,
            },
            'Q' => Piece {
                ptype: PieceType::Queen,
                color: piece_color,
            },
            'B' => Piece {
                ptype: PieceType::Bishop,
                color: piece_color,
            },
            'N' => Piece {
                ptype: PieceType::Knight,
                color: piece_color,
            },
            'R' => Piece {
                ptype: PieceType::Rook,
                color: piece_color,
            },
            'P' => Piece {
                ptype: PieceType::Pawn,
                color: piece_color,
            },
            _ => Piece {
                ptype: PieceType::EmptySpace,
                color: piece_color,
            },
        }
    }

    fn get_ascii(&self) -> char {
        match self.ptype {
            PieceType::King => 'K',
            PieceType::Queen => 'Q',
            PieceType::Bishop => 'B',
            PieceType::Knight => 'N',
            PieceType::Rook => 'R',
            PieceType::Pawn => 'P',
            PieceType::EmptySpace => ' ',
        }
    }
}

// --------------------------------------------------------------------------
// CHESS BOARD
// --------------------------------------------------------------------------

struct ChessBoard {
    map: [Piece; 64],
}

impl ChessBoard {
    fn new() -> ChessBoard {
        ChessBoard {
            map: [
                // Rank 8
                Piece::new('R', PieceColor::Black),
                Piece::new('N', PieceColor::Black),
                Piece::new('B', PieceColor::Black),
                Piece::new('Q', PieceColor::Black),
                Piece::new('K', PieceColor::Black),
                Piece::new('B', PieceColor::Black),
                Piece::new('N', PieceColor::Black),
                Piece::new('R', PieceColor::Black),
                // Rank 7
                Piece::new('P', PieceColor::Black),
                Piece::new('P', PieceColor::Black),
                Piece::new('P', PieceColor::Black),
                Piece::new('P', PieceColor::Black),
                Piece::new('P', PieceColor::Black),
                Piece::new('P', PieceColor::Black),
                Piece::new('P', PieceColor::Black),
                Piece::new('P', PieceColor::Black),
                // Rank 6
                Piece::new(' ', PieceColor::Empty),
                Piece::new(' ', PieceColor::Empty),
                Piece::new(' ', PieceColor::Empty),
                Piece::new(' ', PieceColor::Empty),
                Piece::new(' ', PieceColor::Empty),
                Piece::new(' ', PieceColor::Empty),
                Piece::new(' ', PieceColor::Empty),
                Piece::new(' ', PieceColor::Empty),
                // Rank 5
                Piece::new(' ', PieceColor::Empty),
                Piece::new(' ', PieceColor::Empty),
                Piece::new(' ', PieceColor::Empty),
                Piece::new(' ', PieceColor::Empty),
                Piece::new(' ', PieceColor::Empty),
                Piece::new(' ', PieceColor::Empty),
                Piece::new(' ', PieceColor::Empty),
                Piece::new(' ', PieceColor::Empty),
                // Rank 4
                Piece::new(' ', PieceColor::Empty),
                Piece::new(' ', PieceColor::Empty),
                Piece::new(' ', PieceColor::Empty),
                Piece::new(' ', PieceColor::Empty),
                Piece::new(' ', PieceColor::Empty),
                Piece::new(' ', PieceColor::Empty),
                Piece::new(' ', PieceColor::Empty),
                Piece::new(' ', PieceColor::Empty),
                // Rank 3
                Piece::new(' ', PieceColor::Empty),
                Piece::new(' ', PieceColor::Empty),
                Piece::new(' ', PieceColor::Empty),
                Piece::new(' ', PieceColor::Empty),
                Piece::new(' ', PieceColor::Empty),
                Piece::new(' ', PieceColor::Empty),
                Piece::new(' ', PieceColor::Empty),
                Piece::new(' ', PieceColor::Empty),
                // Rank 2
                Piece::new('P', PieceColor::White),
                Piece::new('P', PieceColor::White),
                Piece::new('P', PieceColor::White),
                Piece::new('P', PieceColor::White),
                Piece::new('P', PieceColor::White),
                Piece::new('P', PieceColor::White),
                Piece::new('P', PieceColor::White),
                Piece::new('P', PieceColor::White),
                // Rank 1
                Piece::new('R', PieceColor::White),
                Piece::new('N', PieceColor::White),
                Piece::new('B', PieceColor::White),
                Piece::new('Q', PieceColor::White),
                Piece::new('K', PieceColor::White),
                Piece::new('B', PieceColor::White),
                Piece::new('N', PieceColor::White),
                Piece::new('R', PieceColor::White),
            ],
        }
    }

    fn display(&self, player: &PieceColor) {
        println!("\n\n   *---+---+---+---+---+---+---+---*");
        for i in 0..8 {
            let rank = if let PieceColor::White = player {
                7 - i
            } else {
                i
            };
            print!(" {} |", rank + 1);

            for j in 0..8 {
                let file = if let PieceColor::White = player {
                    j
                } else {
                    7 - j
                };
                print!(" {} |", self.map[8 * rank + file].get_ascii());
            }
            println!("");
            println!("   *---+---+---+---+---+---+---+---*");
        }

        // print the file index
        if let PieceColor::White = player {
            println!("     a   b   c   d   e   f   g   h  ");
        } else {
            println!("     h   g   f   e   d   c   b   a  ");
        }
    }

    fn take_turn(&self, player: &PieceColor) {
        println!("\nEnter Action: ");
        let mut action = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read input");
        println!("Entered action is {}", action);
    }
}

// --------------------------------------------------------------------------
// MAIN GAME LOOP
// --------------------------------------------------------------------------

fn main() {
    println!("  ___ _                 ___ _    ___ ");
    println!(" / __| |_  ___ ______  / __| |  |_ _|");
    println!("| (__| ' \\/ -_|_-<_-< | (__| |__ | | ");
    println!(" \\___|_||_\\___/__/__/  \\___|____|___|");
                                         
    let board = ChessBoard::new();
    let mut player_turn = PieceColor::White;

    loop {
        // Primary Game Loop Logic
        board.display(&player_turn);
        board.take_turn(&player_turn);

        // Turn Update logic
        if let PieceColor::White = player_turn {
            player_turn = PieceColor::Black;
        } else {
            player_turn = PieceColor::White;
        }
    }
}
