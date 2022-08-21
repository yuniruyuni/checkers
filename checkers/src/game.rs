use crate::board::Board;

pub struct Game {
    red: Board, // 1st player piece existence.
    blk: Board, // 2nd player piece existence.
    king: Board, // the piece is king or pone.
}
