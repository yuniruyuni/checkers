use crate::player::Player;
use crate::board::Board;
use crate::mv::Move;
use crate::dir::Dir;

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

impl Game {
    /// moves() enumerates all next move candidates.
    pub fn moves(&self) -> impl Iterator<Item = Move> {
        let jumpables = self.jumpables();
        let jumped = jumpables != Board::empty();
        let movables = if jumped { jumpables } else { self.movables() };

        let cloned = self.clone();

        movables
            .iter()
            .flat_map(move |p| Move::cands(p, jumped))
            .filter(move |m| cloned.valid(m))
    }

    fn valid(&self, m: &Move) -> bool {
        let king = m.src.is(self.king);
        let dir_ok = m.dir.valid(self.side, king, m.src);

        let board = m.src.board();

        let gap = self.gap();
        let op = if self.side == Player::BLK { self.red } else { self.blk };

        let gap_ok = if m.jump {
            let first = m.dir.apply(board);
            let second = m.dir.apply(first);

            let has_op = (first & op) != Board::empty();
            let has_gap = (second & gap) != Board::empty();
            has_op && has_gap
        } else {
            let first = m.dir.apply(board);
            let has_gap = (first & gap) != Board::empty();
            has_gap
        };

        dir_ok && gap_ok
    }

    fn movables(&self) -> Board {
        match self.side {
            Player::BLK => self.blk_movables(),
            Player::RED => self.red_movables(),
        }
    }

    fn gap(&self) -> Board {
        !(self.blk | self.red)
    }

    fn blk_movables(&self) -> Board {
        let gap = self.gap();

        let b = self.blk;
        let bk = self.blk & self.king;

        let mut movables = Board::empty();
        movables |= (Dir::BackwardLeft.apply(gap) | Dir::BackwardRight.apply(gap)) & b;
        movables |= (Dir::ForwardLeft.apply(gap) | Dir::ForwardRight.apply(gap)) & bk;

        movables
    }

    fn red_movables(&self) -> Board {
        let gap = self.gap();

        let r = self.red;
        let rk = self.red & self.king;

        let mut movables = Board::empty();
        movables |= (Dir::ForwardLeft.apply(gap) | Dir::ForwardRight.apply(gap)) & r;
        movables |= (Dir::BackwardLeft.apply(gap) | Dir::BackwardRight.apply(gap)) & rk;

        movables
    }

    fn jumpables(&self) -> Board {
        match self.side {
            Player::BLK => self.blk_jumpables(),
            Player::RED => self.red_jumpables(),
        }
    }

    fn blk_jumpables(&self) -> Board {
        let gap = self.gap();

        let b = self.blk;
        let bk = self.blk & self.king;

        let mut movables = Board::empty();

        let tmp = Dir::BackwardLeft.apply(gap) & self.red;
        movables |= Dir::BackwardLeft.apply(tmp) & b;

        let tmp = Dir::BackwardRight.apply(gap) & self.red;
        movables |= Dir::BackwardRight.apply(tmp) & b;

        let tmp = Dir::ForwardLeft.apply(gap) & self.red;
        movables |= Dir::ForwardLeft.apply(tmp) & bk;

        let tmp = Dir::ForwardRight.apply(gap) & self.red;
        movables |= Dir::ForwardRight.apply(tmp) & bk;

        movables
    }

    fn red_jumpables(&self) -> Board {
        let gap = self.gap();

        let r = self.red;
        let rk = self.red & self.king;

        let mut movables = Board::empty();

        let tmp = Dir::ForwardLeft.apply(gap) & self.blk;
        movables |= Dir::ForwardLeft.apply(tmp) & r;

        let tmp = Dir::ForwardRight.apply(gap) & self.blk;
        movables |= Dir::ForwardRight.apply(tmp) & r;

        let tmp = Dir::BackwardLeft.apply(gap) & self.blk;
        movables |= Dir::BackwardLeft.apply(tmp) & rk;

        let tmp = Dir::BackwardRight.apply(gap) & self.blk;
        movables |= Dir::BackwardRight.apply(tmp) & rk;

        movables
    }
}

#[cfg(test)]
pub mod testutil {
    use super::*;

    use unindent::unindent;
    use crate::pos::Pos;
    use crate::player::Player;

    pub fn game(side: Player, s: &str) -> Game {
        let s = unindent(s);
        let mut game = Game::default();
        game.side = side;

        let lines = s.split("\n");
        for (y, line) in lines.enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos= Pos::graphical(x as u8, y as u8).and_then(|pos| {
                    match c {
                        'b' => Some((Player::BLK, false, pos)),
                        'B' => Some((Player::BLK, true, pos)),
                        'r' => Some((Player::RED, false, pos)),
                        'R' => Some((Player::RED, true, pos)),
                        _ => None,
                    }
                });
                match pos {
                    Some((Player::BLK, _, pos)) => game.blk |= pos.board(),
                    Some((Player::RED, _, pos)) => game.red |= pos.board(),
                    _ => (),
                };
                match pos {
                    Some((_, true, pos)) => game.king |= pos.board(),
                    _ => (),
                };
            }
        }

        game
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use crate::pos::Pos;

    #[test]
    fn moves_enumerate_all_move_candidates() {
        let cases = [
            (
                "enumerate black pone's moves",
                Player::BLK,
                r"
                    _._._._.
                    ._._._._
                    _._._._.
                    ._._b_._
                    _._._._.
                    ._._._._
                    _._._._.
                    ._._._._
                ",
                vec![
                    Move{src: Pos::new(1, 4), dir: Dir::ForwardRight, jump: false, },
                    Move{src: Pos::new(1, 4), dir: Dir::ForwardLeft, jump: false, },
                ],
            ),
            (
                "enumerate black king's moves",
                Player::BLK,
                r"
                    _._._._.
                    ._._._._
                    _._._._.
                    ._._B_._
                    _._._._.
                    ._._._._
                    _._._._.
                    ._._._._
                ",
                vec![
                    Move{src: Pos::new(1, 4), dir: Dir::ForwardRight, jump: false, },
                    Move{src: Pos::new(1, 4), dir: Dir::ForwardLeft, jump: false, },
                    Move{src: Pos::new(1, 4), dir: Dir::BackwardLeft, jump: false, },
                    Move{src: Pos::new(1, 4), dir: Dir::BackwardRight, jump: false, },
                ],
            ),
            (
                "enumerate red pone's moves",
                Player::RED,
                r"
                    _._._._.
                    ._._._._
                    _._._._.
                    ._._r_._
                    _._._._.
                    ._._._._
                    _._._._.
                    ._._._._
                ",
                vec![
                    Move{src: Pos::new(1, 4), dir: Dir::BackwardLeft, jump: false, },
                    Move{src: Pos::new(1, 4), dir: Dir::BackwardRight, jump: false, },
                ],
            ),
            (
                "enumerate red king's moves",
                Player::RED,
                r"
                    _._._._.
                    ._._._._
                    _._._._.
                    ._._R_._
                    _._._._.
                    ._._._._
                    _._._._.
                    ._._._._
                ",
                vec![
                    Move{src: Pos::new(1, 4), dir: Dir::ForwardRight, jump: false, },
                    Move{src: Pos::new(1, 4), dir: Dir::ForwardLeft, jump: false, },
                    Move{src: Pos::new(1, 4), dir: Dir::BackwardLeft, jump: false, },
                    Move{src: Pos::new(1, 4), dir: Dir::BackwardRight, jump: false, },
                ],
            ),
            (
                "enumerate king's moves on bottom",
                Player::BLK,
                r"
                    _._._._.
                    ._._._._
                    _._._._.
                    ._._._._
                    _._._._.
                    ._._._._
                    _._._._.
                    ._._._B_
                ",
                vec![
                    Move{src: Pos::new(0, 0), dir: Dir::ForwardRight, jump: false, },
                    Move{src: Pos::new(0, 0), dir: Dir::ForwardLeft, jump: false, },
                ],
            ),
            (
                "enumerate king's moves on most right",
                Player::BLK,
                r"
                    _._._._.
                    ._._._._
                    _._._._.
                    ._._._._
                    _._._._.
                    ._._._._
                    _._._._B
                    ._._._._
                ",
                vec![
                    Move{src: Pos::new(0, 1), dir: Dir::ForwardLeft, jump: false, },
                    Move{src: Pos::new(0, 1), dir: Dir::BackwardLeft, jump: false, },
                ],
            ),
            (
                "enumerate king's moves on most left",
                Player::BLK,
                r"
                    _._._._.
                    ._._._._
                    _._._._.
                    ._._._._
                    _._._._.
                    B_._._._
                    _._._._.
                    ._._._._
                ",
                vec![
                    Move{src: Pos::new(3, 2), dir: Dir::ForwardRight, jump: false, },
                    Move{src: Pos::new(3, 2), dir: Dir::BackwardRight, jump: false, },
                ],
            ),
            (
                "enumerate king's moves on most top right",
                Player::BLK,
                r"
                    _._._._B
                    ._._._._
                    _._._._.
                    ._._._._
                    _._._._.
                    ._._._._
                    _._._._.
                    ._._._._
                ",
                vec![
                    Move{src: Pos::new(0, 7), dir: Dir::BackwardLeft, jump: false, },
                ],
            ),
            (
                "enumerate jump over a enemy",
                Player::BLK,
                r"
                    _._._._.
                    ._._._._
                    _._._._.
                    ._._r_._
                    _._B_._.
                    ._._._._
                    _._._._.
                    ._._._._
                ",
                vec![
                    Move{src: Pos::new(2, 3), dir: Dir::ForwardRight, jump: true, },
                ],
            ),
            (
                "enumerate jump over many enemies",
                Player::BLK,
                r"
                    _._._._.
                    ._._._._
                    _._._._.
                    ._r_r_._
                    _._B_._.
                    ._r_r_._
                    _._._._.
                    ._._._._
                ",
                vec![
                    Move{src: Pos::new(2, 3), dir: Dir::ForwardRight, jump: true, },
                    Move{src: Pos::new(2, 3), dir: Dir::ForwardLeft, jump: true, },
                    Move{src: Pos::new(2, 3), dir: Dir::BackwardLeft, jump: true, },
                    Move{src: Pos::new(2, 3), dir: Dir::BackwardRight, jump: true, },
                ],
            ),
            (
                "don't enumerate jump overun",
                Player::BLK,
                r"
                    _._._._.
                    ._._._._
                    _._._._r
                    ._._._B_
                    _._._._r
                    ._._._._
                    _._._._.
                    ._._._._
                ",
                vec![
                    Move{src: Pos::new(0, 4), dir: Dir::ForwardLeft, jump: false, },
                    Move{src: Pos::new(0, 4), dir: Dir::BackwardLeft, jump: false, },
                ],
            ),
            (
                "enumerate all movable piece's moves",
                Player::BLK,
                r"
                    _._._._.
                    ._._._._
                    _._._._.
                    ._b_._._
                    _._._._.
                    ._._B_._
                    _._._._.
                    ._._._._
                ",
                vec![
                    Move{src: Pos::new(1, 2), dir: Dir::ForwardRight, jump: false, },
                    Move{src: Pos::new(1, 2), dir: Dir::ForwardLeft, jump: false, },
                    Move{src: Pos::new(1, 2), dir: Dir::BackwardLeft, jump: false, },
                    Move{src: Pos::new(1, 2), dir: Dir::BackwardRight, jump: false, },
                    Move{src: Pos::new(2, 4), dir: Dir::ForwardRight, jump: false, },
                    Move{src: Pos::new(2, 4), dir: Dir::ForwardLeft, jump: false, },
                ],
            ),
            (
                "avoid move for occupied cell",
                Player::BLK,
                r"
                    _._._._.
                    ._._._._
                    _._._._.
                    ._b_._._
                    _._b_._.
                    ._._._._
                    _._._._.
                    ._._._._
                ",
                vec![
                    Move{src: Pos::new(2, 3), dir: Dir::ForwardRight, jump: false, },
                    Move{src: Pos::new(2, 4), dir: Dir::ForwardRight, jump: false, },
                    Move{src: Pos::new(2, 4), dir: Dir::ForwardLeft, jump: false, },
                ],
            ),
        ];

        for (msg, player, game, mut expects) in cases {
            let game = testutil::game(player, game);

            let mut actuals: Vec<Move> = game.moves().collect();
            expects.sort();
            actuals.sort();
            assert_eq!(expects.len(), actuals.len(), "{}: count of moves are different", msg);
            for (expect, actual) in expects.iter().zip(actuals.iter()) {
                assert_eq!(expect, actual, "{}", msg);
            }
        }
    }
}
