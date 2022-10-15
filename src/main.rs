/*    ________                      ________    ____
 *   / ____/ /_  ___  __________   / ____/ /   /  _/
 *  / /   / __ \/ _ \/ ___/ ___/  / /   / /    / /
 * / /___/ / / /  __(__  |__  )  / /___/ /____/ /
 * \____/_/ /_/\___/____/____/   \____/_____/___/
 *                          - Amlesh Sivanantham
 */

use std::io;

#[derive(Debug)]
enum SystemError {
    InvalidPosition(char, char),
    BadActionString(String),
    PieceMissing,
    InvalidPieceMove,
}

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

#[derive(Copy, Clone)]
struct Position {
    file: i8, // a .. h -> 0 .. 7 ; inner dim
    rank: i8, // 1 .. 8 -> 0 .. 7 ; outer dim
}

impl Position {
    fn new(position: [char; 2]) -> Result<Position, SystemError> {
        let file = position[0];
        let rank = position[1];
        let rank_num: i8 = if rank >= '1' && rank <= '8' {
            rank as i8 - '1' as i8
        } else {
            return Result::Err(SystemError::InvalidPosition(file, rank));
        };
        let file_num: i8 = if file >= 'a' && file <= 'h' {
            file as i8 - 'a' as i8
        } else if file >= 'A' && file <= 'H' {
            file as i8 - 'A' as i8
        } else {
            return Result::Err(SystemError::InvalidPosition(file, rank));
        };
        Result::Ok(Position {
            rank: rank_num,
            file: file_num,
        })
    }
}

#[derive(Copy, Clone)]
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
    fn get_color(&self) -> &'static str;
    fn validate_move(&self, start: Position, end: Position) -> bool {
        true
    }
    fn _validate_rook_like_move(&self, start: Position, end: Position) -> bool {
        if (start.rank == end.rank) && (start.file - end.file).abs() >= 1 {
            true
        } else if (start.file == end.file) && (start.rank - end.rank).abs() >= 1 {
            true
        } else {
            false
        }
    }
    fn _validate_bishop_like_move(&self, start: Position, end: Position) -> bool {
        if (start.rank - end.rank).abs() == (start.file - end.file).abs() {
            true
        } else {
            false
        }
    }
    fn _validate_queen_like_move(&self, start: Position, end: Position) -> bool {
        if self._validate_rook_like_move(start, end) {
            true
        } else if self._validate_bishop_like_move(start, end) {
            true
        } else {
            false
        }
    }
}

struct King {
    color: PieceColor,
}

impl ChessPiece for King {
    fn get_icon(&self) -> char {
        'K'
    }
    fn get_color(&self) -> &'static str {
        self.color.as_str()
    }
    fn validate_move(&self, start: Position, end: Position) -> bool {
        if (start.rank - end.rank).abs() <= 1 && (start.file - end.file).abs() <= 1 {
            true
        } else {
            false
        }
    }
}

struct Queen {
    color: PieceColor,
}
impl ChessPiece for Queen {
    fn get_icon(&self) -> char {
        'Q'
    }
    fn get_color(&self) -> &'static str {
        self.color.as_str()
    }
    fn validate_move(&self, start: Position, end: Position) -> bool {
        self._validate_queen_like_move(start, end)
    }
}

struct Bishop {
    color: PieceColor,
}

impl ChessPiece for Bishop {
    fn get_icon(&self) -> char {
        'i'
    }
    fn get_color(&self) -> &'static str {
        self.color.as_str()
    }
    fn validate_move(&self, start: Position, end: Position) -> bool {
        self._validate_bishop_like_move(start, end)
    }
}

struct Knight {
    color: PieceColor,
}

impl ChessPiece for Knight {
    fn get_icon(&self) -> char {
        'f'
    }
    fn get_color(&self) -> &'static str {
        self.color.as_str()
    }
}

struct Rook {
    color: PieceColor,
}

impl ChessPiece for Rook {
    fn get_icon(&self) -> char {
        '#'
    }
    fn get_color(&self) -> &'static str {
        self.color.as_str()
    }
    fn validate_move(&self, start: Position, end: Position) -> bool {
        self._validate_rook_like_move(start, end)
    }
}

struct Pawn {
    color: PieceColor,
}

impl ChessPiece for Pawn {
    fn get_icon(&self) -> char {
        'x'
    }
    fn get_color(&self) -> &'static str {
        self.color.as_str()
    }
    fn validate_move(&self, start: Position, end: Position) -> bool {
        let rank_direction = match self.color {
            PieceColor::White => 1,
            PieceColor::Black => -1,
        };
        if end.rank - start.rank == rank_direction && (start.file - end.file).abs() <= 1 {
            true
        } else {
            false
        }
    }
}

// --------------------------------------------------------------------------
// CHESS BOARD
// --------------------------------------------------------------------------

enum GameState {
    InProgress,
    Check,
    Checkmate,
    Stalemate,
}

struct ChessBoard {
    map: [[Option<Box<dyn ChessPiece>>; 8]; 8],
}

impl ChessBoard {
    fn new() -> ChessBoard {
        let mut chess_board = ChessBoard {
            map: Default::default(),
        };
        for i in 0..8 {
            chess_board.map[1][i] = Some(Box::new(Pawn {
                color: PieceColor::White,
            }));
            chess_board.map[6][i] = Some(Box::new(Pawn {
                color: PieceColor::Black,
            }));
        }
        for i in [0, 7] {
            let piece_color = if i == 0 {
                PieceColor::White
            } else {
                PieceColor::Black
            };
            chess_board.map[i][0] = Some(Box::new(Rook { color: piece_color }));
            chess_board.map[i][1] = Some(Box::new(Knight { color: piece_color }));
            chess_board.map[i][2] = Some(Box::new(Bishop { color: piece_color }));
            chess_board.map[i][3] = Some(Box::new(Queen { color: piece_color }));
            chess_board.map[i][4] = Some(Box::new(King { color: piece_color }));
            chess_board.map[i][5] = Some(Box::new(Bishop { color: piece_color }));
            chess_board.map[i][6] = Some(Box::new(Knight { color: piece_color }));
            chess_board.map[i][7] = Some(Box::new(Rook { color: piece_color }));
        }
        return chess_board;
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
        if let PieceColor::White = player {
            println!("     a   b   c   d   e   f   g   h  ");
        } else {
            println!("     h   g   f   e   d   c   b   a  ");
        }
    }

    fn get_action(&self) -> Result<[Position; 2], SystemError> {
        println!("\nEnter Action: ");
        let mut action = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read input");
        println!("Entered action is {}", action);
        let action_vec: Vec<char> = action.chars().collect();
        if action.len() == 6 && action_vec[2] == ' ' {
            let start = Position::new([action_vec[0], action_vec[1]]);
            let end = Position::new([action_vec[3], action_vec[4]]);
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

    fn execute_action(&self, start: Position, end: Position) -> Result<GameState, SystemError> {
        let board_square = &self.map[start.rank as usize][start.file as usize];
        match board_square {
            Some(piece) => match piece.validate_move(start, end) {
                true => Result::Ok(GameState::InProgress),
                false => Result::Err(SystemError::InvalidPieceMove),
            },
            None => Result::Err(SystemError::PieceMissing),
        }
    }

    fn take_turn(&self, player: &PieceColor) -> Result<GameState, SystemError> {
        let action = self.get_action();
        match action {
            Result::Ok(position_list) => self.execute_action(position_list[0], position_list[1]),
            Result::Err(error) => Result::Err(error),
        }
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

    let chess_board = ChessBoard::new();
    let mut player_turn = PieceColor::White;

    loop {
        // Primary Game Loop Logic
        chess_board.display(&player_turn);
        match chess_board.take_turn(&player_turn) {
            Result::Ok(game_state) => {
                player_turn = match player_turn {
                    PieceColor::White => PieceColor::Black,
                    PieceColor::Black => PieceColor::White,
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
