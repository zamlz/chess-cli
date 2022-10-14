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

trait ChessPiece {
    fn get_icon(&self) -> char;
    fn validate_action(&self, action: (u8, u8)) -> bool {
        true
    }
}

struct King;
impl ChessPiece for King {
    fn get_icon(&self) -> char {
        'K'
    }
}

struct Queen;
impl ChessPiece for Queen {
    fn get_icon(&self) -> char {
        'Q'
    }
}

struct Bishop;
impl ChessPiece for Bishop {
    fn get_icon(&self) -> char {
        'i'
    }
}

struct Knight;
impl ChessPiece for Knight {
    fn get_icon(&self) -> char {
        'f'
    }
}

struct Rook;
impl ChessPiece for Rook {
    fn get_icon(&self) -> char {
        '#'
    }
}

struct Pawn;
impl ChessPiece for Pawn {
    fn get_icon(&self) -> char {
        'x'
    }
}

// --------------------------------------------------------------------------
// CHESS BOARD
// --------------------------------------------------------------------------

struct ChessBoard {
    map: [[Box<dyn ChessPiece>; 8]; 8],
}

impl ChessBoard {
    fn new() -> ChessBoard {
        ChessBoard {
            map: [[Box::new(Pawn {}); 8]; 8],
        }
    }

    fn display(&self, player: &PieceColor) {
        println!("\n\n   *---+---+---+---+---+---+---+---*");
        /*
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
        */
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
