use crate::board::Board;
use crate::dir::Dir;
use crate::mv::Move;
use crate::player::Player;
use crate::pos::Pos;

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub struct Game {
    pub side: Player,         // which side is now considering next move.
    pub jumping: Option<Pos>, // the piece which is now jumping. it will be None if next hand is normal move.
    pub red: Board,           // 1st player piece existence.
    pub blk: Board,           // 2nd player piece existence.
    pub king: Board,          // the piece is king or pone.
}

impl Game {
    /// moves() enumerates all next move candidates.
    pub fn moves(&self) -> impl Iterator<Item = Move> {
        let jumpables = self.jumpables();
        let jumped = jumpables != Board::empty();
        let movables = match (self.jumping, jumped) {
            (Some(pos), true) => jumpables & pos.board(),
            (Some(_), false) => Board::empty(),
            (None, true) => jumpables,
            (None, false) => self.movables(),
        };

        let cloned = self.clone();

        movables
            .actives()
            .flat_map(move |p| Move::cands(p, jumped))
            .filter(move |m| cloned.valid(m))
    }

    /// PROMOTION_MASK is mask for pone's promotion.
    const PROMOTION_MASK: Board = Board::new(0b1111_0000_0000_0000_0000_0000_0000_1111);

    pub fn apply(&self, m: &Move) -> Game {
        let mut g = self.clone();
        let king = &mut g.king;
        let (slf, opp) = match self.side {
            Player::BLK => (&mut g.blk, &mut g.red),
            Player::RED => (&mut g.red, &mut g.blk),
        };

        let is_king = m.src.is(*king);

        let src_mask = !m.src.board();
        *slf &= src_mask;
        *king &= src_mask;

        let dst_mask = m.dst().board();
        let is_promotion = (dst_mask & Self::PROMOTION_MASK) != Board::empty();

        *slf |= dst_mask;
        if is_king || is_promotion {
            *king |= dst_mask;
        }

        if m.jump {
            *opp &= !m.mid().board();
            *king &= !m.mid().board();
            g.jumping = Some(m.dst());
        } else {
            g.side = !g.side;
            g.jumping = None;
        }

        if g.moves().next().is_none() {
            g.side = !g.side;
            g.jumping = None;
        }

        g
    }

    /// winner() returns which player is winner.
    /// if there are no winner, it retruns None.
    pub fn winner(&self) -> Option<Player> {
        match () {
            _ if self.blk == Board::empty() => Some(Player::RED),
            _ if self.red == Board::empty() => Some(Player::BLK),
            _ => None,
        }
    }

    fn valid(&self, m: &Move) -> bool {
        let king = m.src.is(self.king);
        let dir_ok = m.dir.valid(self.side, king, m.src);

        let board = m.src.board();

        let gap = self.gap();
        let op = if self.side == Player::BLK {
            self.red
        } else {
            self.blk
        };

        let gap_ok = if m.jump {
            let first = m.dir.apply(board);
            let second = m.dir.apply(first);

            let has_op = (first & op) != Board::empty();
            let has_gap = (second & gap) != Board::empty();
            has_op && has_gap
        } else {
            let first = m.dir.apply(board);
            (first & gap) != Board::empty()
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

    use crate::player::Player;
    use crate::pos::Pos;
    use unindent::unindent;

    pub fn game(side: Player, jumping: Option<Pos>, s: &str) -> Game {
        let s = unindent(s);
        let mut game = Game::default();
        game.side = side;
        game.jumping = jumping;

        let lines = s.split("\n");
        for (y, line) in lines.enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = Pos::graphical(x as u8, y as u8).and_then(|pos| match c {
                    'b' => Some((Player::BLK, false, pos)),
                    'B' => Some((Player::BLK, true, pos)),
                    'r' => Some((Player::RED, false, pos)),
                    'R' => Some((Player::RED, true, pos)),
                    _ => None,
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
    use crate::pos::Pos;
    use pretty_assertions::assert_eq;

    #[test]
    fn moves_enumerate_all_move_candidates() {
        let cases = [
            (
                "enumerate black pone's moves",
                Player::BLK,
                None,
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
                    Move {
                        src: Pos::new(1, 4),
                        dir: Dir::ForwardRight,
                        jump: false,
                    },
                    Move {
                        src: Pos::new(1, 4),
                        dir: Dir::ForwardLeft,
                        jump: false,
                    },
                ],
            ),
            (
                "enumerate black king's moves",
                Player::BLK,
                None,
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
                    Move {
                        src: Pos::new(1, 4),
                        dir: Dir::ForwardRight,
                        jump: false,
                    },
                    Move {
                        src: Pos::new(1, 4),
                        dir: Dir::ForwardLeft,
                        jump: false,
                    },
                    Move {
                        src: Pos::new(1, 4),
                        dir: Dir::BackwardLeft,
                        jump: false,
                    },
                    Move {
                        src: Pos::new(1, 4),
                        dir: Dir::BackwardRight,
                        jump: false,
                    },
                ],
            ),
            (
                "enumerate red pone's moves",
                Player::RED,
                None,
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
                    Move {
                        src: Pos::new(1, 4),
                        dir: Dir::BackwardLeft,
                        jump: false,
                    },
                    Move {
                        src: Pos::new(1, 4),
                        dir: Dir::BackwardRight,
                        jump: false,
                    },
                ],
            ),
            (
                "enumerate red king's moves",
                Player::RED,
                None,
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
                    Move {
                        src: Pos::new(1, 4),
                        dir: Dir::ForwardRight,
                        jump: false,
                    },
                    Move {
                        src: Pos::new(1, 4),
                        dir: Dir::ForwardLeft,
                        jump: false,
                    },
                    Move {
                        src: Pos::new(1, 4),
                        dir: Dir::BackwardLeft,
                        jump: false,
                    },
                    Move {
                        src: Pos::new(1, 4),
                        dir: Dir::BackwardRight,
                        jump: false,
                    },
                ],
            ),
            (
                "enumerate king's moves on bottom",
                Player::BLK,
                None,
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
                    Move {
                        src: Pos::new(0, 0),
                        dir: Dir::ForwardRight,
                        jump: false,
                    },
                    Move {
                        src: Pos::new(0, 0),
                        dir: Dir::ForwardLeft,
                        jump: false,
                    },
                ],
            ),
            (
                "enumerate king's moves on most right",
                Player::BLK,
                None,
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
                    Move {
                        src: Pos::new(0, 1),
                        dir: Dir::ForwardLeft,
                        jump: false,
                    },
                    Move {
                        src: Pos::new(0, 1),
                        dir: Dir::BackwardLeft,
                        jump: false,
                    },
                ],
            ),
            (
                "enumerate king's moves on most left",
                Player::BLK,
                None,
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
                    Move {
                        src: Pos::new(3, 2),
                        dir: Dir::ForwardRight,
                        jump: false,
                    },
                    Move {
                        src: Pos::new(3, 2),
                        dir: Dir::BackwardRight,
                        jump: false,
                    },
                ],
            ),
            (
                "enumerate king's moves on most top right",
                Player::BLK,
                None,
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
                vec![Move {
                    src: Pos::new(0, 7),
                    dir: Dir::BackwardLeft,
                    jump: false,
                }],
            ),
            (
                "enumerate jump over a enemy",
                Player::BLK,
                None,
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
                vec![Move {
                    src: Pos::new(2, 3),
                    dir: Dir::ForwardRight,
                    jump: true,
                }],
            ),
            (
                "enumerate jump over many enemies",
                Player::BLK,
                None,
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
                    Move {
                        src: Pos::new(2, 3),
                        dir: Dir::ForwardRight,
                        jump: true,
                    },
                    Move {
                        src: Pos::new(2, 3),
                        dir: Dir::ForwardLeft,
                        jump: true,
                    },
                    Move {
                        src: Pos::new(2, 3),
                        dir: Dir::BackwardLeft,
                        jump: true,
                    },
                    Move {
                        src: Pos::new(2, 3),
                        dir: Dir::BackwardRight,
                        jump: true,
                    },
                ],
            ),
            (
                "don't enumerate jump overun",
                Player::BLK,
                None,
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
                    Move {
                        src: Pos::new(0, 4),
                        dir: Dir::ForwardLeft,
                        jump: false,
                    },
                    Move {
                        src: Pos::new(0, 4),
                        dir: Dir::BackwardLeft,
                        jump: false,
                    },
                ],
            ),
            (
                "enumerate all movable piece's moves",
                Player::BLK,
                None,
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
                    Move {
                        src: Pos::new(1, 2),
                        dir: Dir::ForwardRight,
                        jump: false,
                    },
                    Move {
                        src: Pos::new(1, 2),
                        dir: Dir::ForwardLeft,
                        jump: false,
                    },
                    Move {
                        src: Pos::new(1, 2),
                        dir: Dir::BackwardLeft,
                        jump: false,
                    },
                    Move {
                        src: Pos::new(1, 2),
                        dir: Dir::BackwardRight,
                        jump: false,
                    },
                    Move {
                        src: Pos::new(2, 4),
                        dir: Dir::ForwardRight,
                        jump: false,
                    },
                    Move {
                        src: Pos::new(2, 4),
                        dir: Dir::ForwardLeft,
                        jump: false,
                    },
                ],
            ),
            (
                "avoid move for occupied cell",
                Player::BLK,
                None,
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
                    Move {
                        src: Pos::new(2, 3),
                        dir: Dir::ForwardRight,
                        jump: false,
                    },
                    Move {
                        src: Pos::new(2, 4),
                        dir: Dir::ForwardRight,
                        jump: false,
                    },
                    Move {
                        src: Pos::new(2, 4),
                        dir: Dir::ForwardLeft,
                        jump: false,
                    },
                ],
            ),
            (
                "jumping piece cannot continue jump",
                Player::BLK,
                Some(Pos::new(2, 3)),
                r"
                    _._._._.
                    ._._._._
                    _._b_._.
                    ._._._._
                    _._b_._.
                    ._._._._
                    _._._._.
                    ._._._._
                ",
                vec![],
            ),
            (
                "Red can jump black",
                Player::RED,
                None,
                r"
                    _._._._.
                    ._._._._
                    _._._._.
                    ._._._._
                    _r_._._.
                    ._b_._._
                    _._._._.
                    ._._._._
                ",
                vec![Move {
                    src: Pos::new(3, 3),
                    dir: Dir::BackwardRight,
                    jump: true,
                }],
            ),
        ];

        for (msg, player, jumping, game, mut expects) in cases {
            let game = testutil::game(player, jumping, game);

            let mut actuals: Vec<Move> = game.moves().collect();
            expects.sort();
            actuals.sort();
            assert_eq!(
                expects.len(),
                actuals.len(),
                "{}: count of moves are different",
                msg
            );
            for (expect, actual) in expects.iter().zip(actuals.iter()) {
                assert_eq!(expect, actual, "{}", msg);
            }
        }
    }

    #[test]
    fn apply_moves_correct_piece() {
        let cases = [
            (
                "Apply ForwardRight move",
                Move {
                    src: Pos::new(2, 1),
                    dir: Dir::ForwardRight,
                    jump: false,
                },
                Player::BLK,
                None,
                r"
                    _._._._.
                    ._._B_R_
                    _b_._._.
                    ._._._._
                    _._._._.
                    ._._._._
                    _._b_._.
                    ._._._._
                ",
                Player::RED,
                None,
                r"
                    _._._._.
                    ._._B_R_
                    _b_._._.
                    ._._._._
                    _._._._.
                    ._._b_._
                    _._._._.
                    ._._._._
                ",
            ),
            (
                "Apply ForwardRight jump move and continue play for black",
                Move {
                    src: Pos::new(2, 1),
                    dir: Dir::ForwardRight,
                    jump: true,
                },
                Player::BLK,
                None,
                r"
                    _._._._.
                    ._B_R_._
                    _._._._.
                    ._._r_._
                    _._r_._.
                    ._._r_._
                    _._b_._.
                    ._._._._
                ",
                Player::BLK,
                Some(Pos::new(1, 3)),
                r"
                    _._._._.
                    ._B_R_._
                    _._._._.
                    ._._r_._
                    _._r_b_.
                    ._._._._
                    _._._._.
                    ._._._._
                ",
            ),
            (
                "Apply BackwardRight jump move",
                Move {
                    src: Pos::new(2, 1),
                    dir: Dir::BackwardRight,
                    jump: false,
                },
                Player::BLK,
                None,
                r"
                    _._._._.
                    ._._._._
                    _b_._._.
                    ._._._._
                    _._._._.
                    ._._._._
                    _._b_._.
                    ._._._._
                ",
                Player::BLK,
                None,
                r"
                    _._._._.
                    ._._._._
                    _b_._._.
                    ._._._._
                    _._._._.
                    ._._._._
                    _._._._.
                    ._._B_._
                ",
            ),
            (
                "Apply ForwardRight move for king for black",
                Move {
                    src: Pos::new(2, 1),
                    dir: Dir::ForwardRight,
                    jump: false,
                },
                Player::BLK,
                None,
                r"
                    _._._._.
                    ._._._._
                    _._r_._.
                    ._._._._
                    _._._._.
                    ._._._._
                    _._B_._.
                    ._._._._
                ",
                Player::RED,
                None,
                r"
                    _._._._.
                    ._._._._
                    _._r_._.
                    ._._._._
                    _._._._.
                    ._._B_._
                    _._._._.
                    ._._._._
                ",
            ),
            (
                "Apply ForwardRight move for king for red",
                Move {
                    src: Pos::new(2, 1),
                    dir: Dir::BackwardRight,
                    jump: false,
                },
                Player::RED,
                None,
                r"
                    _._._._.
                    ._._._._
                    _._b_._.
                    ._._._._
                    _._._._.
                    ._._._._
                    _._R_._.
                    ._._._._
                ",
                Player::BLK,
                None,
                r"
                    _._._._.
                    ._._._._
                    _._b_._.
                    ._._._._
                    _._._._.
                    ._._._._
                    _._._._.
                    ._._R_._
                ",
            ),
            (
                "Apply ForwardRight move for king",
                Move {
                    src: Pos::new(2, 1),
                    dir: Dir::ForwardRight,
                    jump: false,
                },
                Player::RED,
                None,
                r"
                    _._._._.
                    ._._._b_
                    _._._._.
                    ._._._._
                    _._._._.
                    ._._._._
                    _._R_._.
                    ._._._._
                ",
                Player::BLK,
                None,
                r"
                    _._._._.
                    ._._._b_
                    _._._._.
                    ._._._._
                    _._._._.
                    ._._R_._
                    _._._._.
                    ._._._._
                ",
            ),
            (
                "Apply ForwardRight jump move and continue play for red",
                Move {
                    src: Pos::new(2, 1),
                    dir: Dir::ForwardRight,
                    jump: true,
                },
                Player::RED,
                None,
                r"
                    _._._._.
                    ._._._._
                    _._._._.
                    ._._b_._
                    _._._._.
                    ._._b_._
                    _._R_._.
                    ._._._._
                ",
                Player::RED,
                Some(Pos::new(1, 3)),
                r"
                    _._._._.
                    ._._._._
                    _._._._.
                    ._._b_._
                    _._._R_.
                    ._._._._
                    _._._._.
                    ._._._._
                ",
            ),
            (
                "Apply ForwardRight jump move without next jumpable move",
                Move {
                    src: Pos::new(2, 1),
                    dir: Dir::ForwardRight,
                    jump: true,
                },
                Player::RED,
                None,
                r"
                    _._._._.
                    ._._._._
                    _._._._.
                    ._._._._
                    _._._._.
                    ._._b_._
                    _._R_._.
                    ._._._._
                ",
                Player::BLK,
                None,
                r"
                    _._._._.
                    ._._._._
                    _._._._.
                    ._._._._
                    _._._R_.
                    ._._._._
                    _._._._.
                    ._._._._
                ",
            ),
            (
                // A test for specific bug, refer:
                // https://clips.twitch.tv/IcyBovineFiddleheadsPanicVis-ab4FWbTA4kBNRZOm
                "Change turn when move forward left for corner",
                Move {
                    src: Pos::new(2, 1),
                    dir: Dir::ForwardLeft,
                    jump: false,
                },
                Player::BLK,
                None,
                r"
                    _._._._.
                    ._._._._
                    _._._._.
                    ._._._._
                    _r_._._.
                    ._._._._
                    _._b_._.
                    ._._._._
                ",
                Player::RED,
                None,
                r"
                    _._._._.
                    ._._._._
                    _._._._.
                    ._._._._
                    _r_._._.
                    ._b_._._
                    _._._._.
                    ._._._._
                ",
            ),
            (
                "BLK pone's promotion",
                Move {
                    src: Pos::new(2, 6),
                    dir: Dir::ForwardLeft,
                    jump: false,
                },
                Player::BLK,
                None,
                r"
                    _._._._.
                    ._b_._._
                    _._._._.
                    ._._._._
                    _._._._.
                    ._r_._._
                    _._._._.
                    ._._._._
                ",
                Player::RED,
                None,
                r"
                    _B_._._.
                    ._._._._
                    _._._._.
                    ._._._._
                    _._._._.
                    ._r_._._
                    _._._._.
                    ._._._._
                ",
            ),
            (
                "RED pone's promotion",
                Move {
                    src: Pos::new(2, 1),
                    dir: Dir::BackwardRight,
                    jump: false,
                },
                Player::RED,
                None,
                r"
                    _._._._.
                    ._._._._
                    _b_._._.
                    ._._._._
                    _._._._.
                    ._._._._
                    _._r_._.
                    ._._._._
                ",
                Player::BLK,
                None,
                r"
                    _._._._.
                    ._._._._
                    _b_._._.
                    ._._._._
                    _._._._.
                    ._._._._
                    _._._._.
                    ._._R_._
                ",
            ),
        ];

        for (msg, m, before_player, before_jumping, before, after_player, after_jumping, after) in
            cases
        {
            let before = testutil::game(before_player, before_jumping, before);
            let expected = testutil::game(after_player, after_jumping, after);
            let actual = before.apply(&m);

            assert_eq!(expected, actual, "{}", msg);
        }
    }

    #[test]
    fn test_checkmate() {
        let cases = [
            (
                "Game is cotninue normally",
                Player::RED,
                r"
                    _._._._.
                    ._b_._._
                    _._._._.
                    ._._R_._
                    _r_._._.
                    ._b_._._
                    _._._r_.
                    ._._._._
                ",
                None,
            ),
            (
                "Game only have black pieces",
                Player::BLK,
                r"
                    _._._._.
                    ._._._._
                    _._._._.
                    ._._B_._
                    _b_._._.
                    ._._._._
                    _._._b_.
                    ._._._._
                ",
                Some(Player::BLK),
            ),
            (
                "Game only have red pieces",
                Player::RED,
                r"
                    _._._._.
                    ._._._._
                    _._._._.
                    ._._R_._
                    _r_._._.
                    ._._._._
                    _._._r_.
                    ._._._._
                ",
                Some(Player::RED),
            ),
        ];

        for (msg, player, game, expected) in cases {
            let game = testutil::game(player, None, game);
            let actual = game.winner();

            assert_eq!(expected, actual, "{}", msg);
        }
    }
}
