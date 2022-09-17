use crate::player::Player;
use crate::board::Board;
use crate::pos::Pos;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dir {
    ForwardRight,
    ForwardLeft,
    BackwardLeft,
    BackwardRight,
}

impl Dir {
    pub fn valid(self, p: Player, king: bool, pos: Pos) -> bool {
        self.valid_piece(p, king) && self.valid_pos(pos)
    }

    #[inline(always)]
    pub fn apply(self, target: Board) -> Board {
        let ls = self.latent();
        ls[0].apply(target) | ls[1].apply(target)
    }

    #[inline(always)]
    fn valid_piece(self, p: Player, king: bool) -> bool {
        match (self, p, king) {
            (_, _, true) => true,
            (Self::ForwardRight, Player::BLK, _) => true,
            (Self::ForwardLeft, Player::BLK, _) => true,
            (Self::BackwardLeft, Player::RED, _) => true,
            (Self::BackwardRight, Player::RED, _) => true,
            (_, _, _) => false,
        }
    }

    #[inline(always)]
    fn valid_pos(self, target: Pos) -> bool {
        let pad = target.y() % 2 == 0;
        let diff = match (self, pad) {
            (Self::ForwardRight, true) => (0, 1),
            (Self::ForwardRight, false) => (-1, 1),
            (Self::ForwardLeft, true) => (1, 1),
            (Self::ForwardLeft, false) => (0, 1),
            (Self::BackwardLeft, true) => (1, -1),
            (Self::BackwardLeft, false) => (0, -1),
            (Self::BackwardRight, true) => (0, -1),
            (Self::BackwardRight, false) => (-1, -1),
        };

        let mx = target.x() as i8 + diff.0;
        let my = target.y() as i8 + diff.1;

        (0 <= mx && mx < 4) && (0 <= my && my < 8)
    }

    #[inline(always)]
    fn latent(self) -> [Latent; 2] {
        match self {
            Self::ForwardRight => [Latent::FA, Latent::FC],
            Self::ForwardLeft => [Latent::FB, Latent::FD],
            Self::BackwardLeft => [Latent::BA, Latent::BC],
            Self::BackwardRight => [Latent::BB, Latent::BD],
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Latent {
    FA, FB, FC, FD,
    BA, BB, BC, BD,
}

impl Latent {
    const MASK_FA: Board = Board::new(0b0000_1111_0000_1111_0000_1111_0000_1111);
    const MASK_BA: Board = Board::new(0b1111_0000_1111_0000_1111_0000_1111_0000);

    const MASK_FB: Board = Board::new(0b0000_0000_1111_0000_1111_0000_1111_0000);
    const MASK_BB: Board = Board::new(0b0000_1111_0000_1111_0000_1111_0000_0000);

    const MASK_FC: Board = Board::new(0b0000_0000_1110_0000_1110_0000_1110_0000);
    const MASK_BC: Board = Board::new(0b0000_0111_0000_0111_0000_0111_0000_0000);

    const MASK_FD: Board = Board::new(0b0000_0111_0000_0111_0000_0111_0000_0111);
    const MASK_BD: Board = Board::new(0b1110_0000_1110_0000_1110_0000_1110_0000);

    #[inline]
    pub fn apply(self, target: Board) -> Board {
        let masked = target & self.mask();
        let diff = self.diff();
        if 0 <= diff { masked << diff } else { masked >> -diff }
    }

    #[inline(always)]
    fn mask(self) -> Board {
        match self {
            Self::FA => Self::MASK_FA,
            Self::BA => Self::MASK_BA,
            Self::FB => Self::MASK_FB,
            Self::BB => Self::MASK_BB,
            Self::FC => Self::MASK_FC,
            Self::BC => Self::MASK_BC,
            Self::FD => Self::MASK_FD,
            Self::BD => Self::MASK_BD,
        }
    }

    #[inline(always)]
    fn diff(self) -> i8 {
        match self {
            Self::FA => 4,
            Self::BA => -4,
            Self::FB => 4,
            Self::BB => -4,
            Self::FC => 3,
            Self::BC => -3,
            Self::FD => 5,
            Self::BD => -5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use crate::board::testutil::board;

    #[test]
    fn apply_direciton() {
        let cases = [
            (
                "Forward move moves desired items",
                Dir::ForwardRight,
                r"
                    _0_0_0_0
                    1_0_0_0_
                    _0_0_0_0
                    0_0_0_0_
                    _0_1_1_0
                    1_1_1_1_
                    _1_1_0_0
                    1_1_1_1_
                ",
                r"
                    _1_0_0_0
                    0_0_0_0_
                    _0_0_0_0
                    0_0_1_1_
                    _1_1_1_1
                    0_1_1_0_
                    _1_1_1_1
                    0_0_0_0_
                ",
            ),
            (
                "Backward move moves desired items",
                Dir::BackwardLeft,
                r"
                    _1_0_0_0
                    0_0_0_0_
                    _0_0_0_0
                    0_0_1_1_
                    _1_1_1_1
                    0_1_1_0_
                    _1_1_1_1
                    0_0_0_0_
                ",
                r"
                    _0_0_0_0
                    1_0_0_0_
                    _0_0_0_0
                    0_0_0_0_
                    _0_1_1_0
                    1_1_1_1_
                    _1_1_0_0
                    1_1_1_1_
                ",
            ),
        ];

        for (msg, dir, before, after) in cases {
            let before = board(before);
            let after = board(after);

            let actual = dir.apply(before);
            assert_eq!(after, actual, "{}", msg);
        }
    }

    #[test]
    fn apply_latent_direciton() {
        let cases = [
            (
                "FA moves desired items",
                Latent::FA,
                r"
                    _0_0_0_0
                    1_0_0_0_
                    _0_0_0_0
                    0_0_0_0_
                    _0_0_0_0
                    1_1_1_1_
                    _0_0_0_0
                    1_1_1_1_
                ",
                r"
                    _1_0_0_0
                    0_0_0_0_
                    _0_0_0_0
                    0_0_0_0_
                    _1_1_1_1
                    0_0_0_0_
                    _1_1_1_1
                    0_0_0_0_
                ",
            ),
            (
                "FA doesn't move not desired items",
                Latent::FA,
                r"
                    _1_0_0_0
                    0_0_0_0_
                    _1_0_0_0
                    0_0_0_0_
                    _0_0_0_0
                    0_0_0_0_
                    _1_1_1_1
                    0_0_0_0_
                ",
                r"
                    _0_0_0_0
                    0_0_0_0_
                    _0_0_0_0
                    0_0_0_0_
                    _0_0_0_0
                    0_0_0_0_
                    _0_0_0_0
                    0_0_0_0_
                ",
            ),
            (
                "FB moves desired items",
                Latent::FB,
                r"
                    _0_0_0_0
                    0_0_0_0_
                    _0_0_0_0
                    0_0_0_0_
                    _1_1_1_1
                    0_0_0_0_
                    _1_1_1_1
                    0_0_0_0_
                ",
                r"
                    _0_0_0_0
                    0_0_0_0_
                    _0_0_0_0
                    1_1_1_1_
                    _0_0_0_0
                    1_1_1_1_
                    _0_0_0_0
                    0_0_0_0_
                ",
            ),
            (
                "FC moves desired items",
                Latent::FC,
                r"
                    _0_0_0_0
                    0_0_0_0_
                    _1_1_1_0
                    0_0_0_0_
                    _1_1_1_0
                    0_0_0_0_
                    _1_1_1_0
                    0_0_0_0_
                ",
                r"
                    _0_0_0_0
                    0_1_1_1_
                    _0_0_0_0
                    0_1_1_1_
                    _0_0_0_0
                    0_1_1_1_
                    _0_0_0_0
                    0_0_0_0_
                ",
            ),
            (
                "FD moves desired items",
                Latent::FD,
                r"
                    _0_0_0_0
                    0_1_1_1_
                    _0_0_0_0
                    0_1_1_1_
                    _0_0_0_0
                    0_1_1_1_
                    _0_0_0_0
                    0_1_1_1_
                ",
                r"
                    _1_1_1_0
                    0_0_0_0_
                    _1_1_1_0
                    0_0_0_0_
                    _1_1_1_0
                    0_0_0_0_
                    _1_1_1_0
                    0_0_0_0_
                ",
            ),
            (
                "FD doesn't moves not desired items",
                Latent::FD,
                r"
                    _1_1_1_1
                    1_0_0_0_
                    _0_0_1_0
                    1_0_0_0_
                    _0_1_0_0
                    1_0_0_0_
                    _0_0_0_0
                    1_0_0_0_
                ",
                r"
                    _0_0_0_0
                    0_0_0_0_
                    _0_0_0_0
                    0_0_0_0_
                    _0_0_0_0
                    0_0_0_0_
                    _0_0_0_0
                    0_0_0_0_
                ",
            ),
            (
                "Backward move moves desired items",
                Latent::BC,
                r"
                    _0_0_0_0
                    0_1_1_1_
                    _0_0_0_0
                    0_1_1_1_
                    _0_0_0_0
                    0_1_1_1_
                    _0_0_0_0
                    0_0_0_0_
                ",
                r"
                    _0_0_0_0
                    0_0_0_0_
                    _1_1_1_0
                    0_0_0_0_
                    _1_1_1_0
                    0_0_0_0_
                    _1_1_1_0
                    0_0_0_0_
                ",
            ),
            (
                "Backward move doesn't move not desired items",
                Latent::BC,
                r"
                    _1_1_1_1
                    1_0_0_0_
                    _0_1_0_0
                    1_0_0_0_
                    _0_1_1_0
                    1_0_0_0_
                    _0_0_0_0
                    1_1_1_1_
                ",
                r"
                    _0_0_0_0
                    0_0_0_0_
                    _0_0_0_0
                    0_0_0_0_
                    _0_0_0_0
                    0_0_0_0_
                    _0_0_0_0
                    0_0_0_0_
                ",
            ),
        ];

        for (msg, dir, before, after) in cases {
            let before = board(before);
            let after = board(after);

            let actual = dir.apply(before);
            assert_eq!(after, actual, "{}", msg);
        }
    }
}
