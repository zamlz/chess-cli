use crate::chess::error::SystemError;

#[derive(Copy, Clone)]
pub struct Position {
    pub file: i8, // a .. h -> 0 .. 7 ; inner dim
    pub rank: i8, // 1 .. 8 -> 0 .. 7 ; outer dim
}

impl Position {
    pub fn new(position: [char; 2]) -> Result<Position, SystemError> {
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
pub enum PieceColor {
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

pub trait ChessPiece {
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

pub struct King {
    pub color: PieceColor,
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

pub struct Queen {
    pub color: PieceColor,
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

pub struct Bishop {
    pub color: PieceColor,
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

pub struct Knight {
    pub color: PieceColor,
}

impl ChessPiece for Knight {
    fn get_icon(&self) -> char {
        'f'
    }
    fn get_color(&self) -> &'static str {
        self.color.as_str()
    }
}

pub struct Rook {
    pub color: PieceColor,
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

pub struct Pawn {
    pub color: PieceColor,
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
