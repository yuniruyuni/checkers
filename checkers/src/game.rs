use crate::player::Player;
use crate::board::Board;

#[derive(
    Debug, Default,
    Clone,
    PartialEq, Eq,
)]
pub struct Game {
    pub side: Player, // which side is now considering next move.
    pub red: Board, // 1st player piece existence.
    pub blk: Board, // 2nd player piece existence.
    pub king: Board, // the piece is king or pone.
}
