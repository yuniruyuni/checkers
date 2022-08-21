use crate::board::Board;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    ForwardLeft,
    ForwardRight,
    BackwardLeft,
    BackwardRight,
}

impl Dir {
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
    use crate::board::tests::board;

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
