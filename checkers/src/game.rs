use crate::player::Player;
use crate::board::Board;

#[derive(
    Debug, Default,
    Clone,
    PartialEq, Eq,
)]
pub struct Game {
    side: Player, // which side is now considering next move.
    red: Board, // 1st player piece existence.
    blk: Board, // 2nd player piece existence.
    king: Board, // the piece is king or pone.
}
