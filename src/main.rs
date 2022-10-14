/*    ________                      ________    ____
 *   / ____/ /_  ___  __________   / ____/ /   /  _/
 *  / /   / __ \/ _ \/ ___/ ___/  / /   / /    / /
 * / /___/ / / /  __(__  |__  )  / /___/ /____/ /
 * \____/_/ /_/\___/____/____/   \____/_____/___/
 *                          - Amlesh Sivanantham
 */

use std::io;

enum ColorString {
    Reset,
    // Foreground colors
    FgBlue,
    FgRed,
    // Background colors
    BgBlack,
    BgWhite,
}

impl ColorString {
    fn as_str(&self) -> &'static str {
        match self {
            ColorString::Reset => "\x1b[0m",
            // Foreground colors
            ColorString::FgBlue => "\x1b[34m",
            ColorString::FgRed => "\x1b[31m",
            // Background colors
            ColorString::BgBlack => "\x1b[40m",
            ColorString::BgWhite => "\x1b[47m",
        }
    }
}

// --------------------------------------------------------------------------
// CHESS PIECE
// --------------------------------------------------------------------------

enum PieceColor {
    White,
    Black,
}

impl PieceColor {
    fn as_str(&self) -> &'static str {
        match self {
            PieceColor::White => "\x1b[34m\x1b[1m",
            PieceColor::Black => "\x1b[31m\x1b[1m",
        }
    }
}

enum ChessPiece {
    EmptySpace,
    King(PieceColor),
    Queen(PieceColor),
    Bishop(PieceColor),
    Knight(PieceColor),
    Rook(PieceColor),
    Pawn(PieceColor),
}

impl ChessPiece {
    fn new(piece: char, color: char) -> ChessPiece {
        let piece_color = match color {
            'W' => PieceColor::White,
            'B' => PieceColor::Black,
            _ => panic!("invalid piece color"),
        };
        match piece {
            'K' => ChessPiece::King(piece_color),
            'Q' => ChessPiece::Queen(piece_color),
            'B' => ChessPiece::Bishop(piece_color),
            'N' => ChessPiece::Knight(piece_color),
            'R' => ChessPiece::Rook(piece_color),
            'P' => ChessPiece::Pawn(piece_color),
            _ => ChessPiece::EmptySpace,
        }
    }

    fn get_token(&self) -> char {
        match self {
            ChessPiece::King(_) => 'K',
            ChessPiece::Queen(_) => 'Q',
            ChessPiece::Bishop(_) => 'i',
            ChessPiece::Knight(_) => 'f',
            ChessPiece::Rook(_) => '#',
            ChessPiece::Pawn(_) => 'x',
            ChessPiece::EmptySpace => ' ',
        }
    }

    fn get_color(&self) -> &'static str {
        match self {
            ChessPiece::King(color) => color.as_str(),
            ChessPiece::Queen(color) => color.as_str(),
            ChessPiece::Bishop(color) => color.as_str(),
            ChessPiece::Knight(color) => color.as_str(),
            ChessPiece::Rook(color) => color.as_str(),
            ChessPiece::Pawn(color) => color.as_str(),
            ChessPiece::EmptySpace => "",
        }
    }

    //fn validate_action(&self, action: (u8, u8)) -> bool;

    //fn execute_action(&self, action: (u8, u8));
}

// --------------------------------------------------------------------------
// CHESS BOARD
// --------------------------------------------------------------------------

struct ChessBoard {
    map: [ChessPiece; 64],
}

impl ChessBoard {
    fn new() -> ChessBoard {
        ChessBoard {
            map: [
                // Rank 8
                ChessPiece::new('R', 'B'),
                ChessPiece::new('N', 'B'),
                ChessPiece::new('B', 'B'),
                ChessPiece::new('Q', 'B'),
                ChessPiece::new('K', 'B'),
                ChessPiece::new('B', 'B'),
                ChessPiece::new('N', 'B'),
                ChessPiece::new('R', 'B'),
                // Rank 7
                ChessPiece::new('P', 'B'),
                ChessPiece::new('P', 'B'),
                ChessPiece::new('P', 'B'),
                ChessPiece::new('P', 'B'),
                ChessPiece::new('P', 'B'),
                ChessPiece::new('P', 'B'),
                ChessPiece::new('P', 'B'),
                ChessPiece::new('P', 'B'),
                // Rank 6
                ChessPiece::EmptySpace,
                ChessPiece::EmptySpace,
                ChessPiece::EmptySpace,
                ChessPiece::EmptySpace,
                ChessPiece::EmptySpace,
                ChessPiece::EmptySpace,
                ChessPiece::EmptySpace,
                ChessPiece::EmptySpace,
                // Rank 5
                ChessPiece::EmptySpace,
                ChessPiece::EmptySpace,
                ChessPiece::EmptySpace,
                ChessPiece::EmptySpace,
                ChessPiece::EmptySpace,
                ChessPiece::EmptySpace,
                ChessPiece::EmptySpace,
                ChessPiece::EmptySpace,
                // Rank 4
                ChessPiece::EmptySpace,
                ChessPiece::EmptySpace,
                ChessPiece::EmptySpace,
                ChessPiece::EmptySpace,
                ChessPiece::EmptySpace,
                ChessPiece::EmptySpace,
                ChessPiece::EmptySpace,
                ChessPiece::EmptySpace,
                // Rank 3
                ChessPiece::EmptySpace,
                ChessPiece::EmptySpace,
                ChessPiece::EmptySpace,
                ChessPiece::EmptySpace,
                ChessPiece::EmptySpace,
                ChessPiece::EmptySpace,
                ChessPiece::EmptySpace,
                ChessPiece::EmptySpace,
                // Rank 2
                ChessPiece::new('P', 'W'),
                ChessPiece::new('P', 'W'),
                ChessPiece::new('P', 'W'),
                ChessPiece::new('P', 'W'),
                ChessPiece::new('P', 'W'),
                ChessPiece::new('P', 'W'),
                ChessPiece::new('P', 'W'),
                ChessPiece::new('P', 'W'),
                // Rank 1
                ChessPiece::new('R', 'W'),
                ChessPiece::new('N', 'W'),
                ChessPiece::new('B', 'W'),
                ChessPiece::new('Q', 'W'),
                ChessPiece::new('K', 'W'),
                ChessPiece::new('B', 'W'),
                ChessPiece::new('N', 'W'),
                ChessPiece::new('R', 'W'),
            ],
        }
    }

    fn display(&self, player: &PieceColor) {
        println!("\n\n   *---+---+---+---+---+---+---+---*");
        for i in 0..8 {
            // compute the real rank
            let rank = if let PieceColor::White = player {
                7 - i
            } else {
                i
            };
            print!(" {} |", rank + 1);

            for j in 0..8 {
                // Compute the real file
                let file = if let PieceColor::White = player {
                    j
                } else {
                    7 - j
                };
                // color time
                if (rank + file) % 2 == 0 {
                    print!("{} ", ColorString::BgBlack.as_str());
                } else {
                    print!("{} ", ColorString::BgWhite.as_str());
                }
                print!("{}", self.map[8 * rank + file].get_color());
                print!("{}", self.map[8 * rank + file].get_token());
                print!(" {}|", ColorString::Reset.as_str());
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
    print!("{}", ColorString::FgBlue.as_str());
    println!("  ___ _                 ___ _    ___ ");
    println!(" / __| |_  ___ ______  / __| |  |_ _|");
    println!("| (__| ' \\/ -_|_-<_-< | (__| |__ | | ");
    println!(" \\___|_||_\\___/__/__/  \\___|____|___|");
    print!("{}", ColorString::FgRed.as_str());
    println!("                              v0.0.1");
    print!("{}", ColorString::Reset.as_str());

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
