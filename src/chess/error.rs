#[derive(Debug)]
pub enum SystemError {
    InvalidPosition(char, char),
    BadActionString(String),
    PieceMissing,
    InvalidPieceMove,
}
