use derive_more::{
    BitAnd, BitAndAssign,
    BitOr, BitOrAssign,
    BitXor, BitXorAssign,
    Shl, ShlAssign,
    Shr, ShrAssign,
    Not,
};

use crate::pos::Pos;

#[derive(
    Debug, Default,
    Clone, Copy,
    PartialEq, Eq,
    BitAnd, BitAndAssign,
    BitOr, BitOrAssign,
    BitXor, BitXorAssign,
    Shl, ShlAssign,
    Shr, ShrAssign,
    Not,
)]
pub struct Board(u32);

impl Board {
    pub const fn new(bits: u32) -> Board {
        Board(bits)
    }

    pub const fn empty() -> Board {
        Board(0)
    }

    /// actives() iterate all active positions.
    pub fn actives(self) -> impl Iterator<Item = Pos> {
        let mut bits = self.0;
        let mut shifted = 0u8;
        std::iter::from_fn(move ||{
            if bits == 0 { return None; }
            let shift = bits.trailing_zeros() as u8;
            bits >>= shift;
            bits &= !1u32;
            shifted += shift;
            Some(Pos::raw(shifted))
        })
    }
}

#[cfg(test)]
pub mod testutil {
    use super::*;

    use unindent::unindent;
    use crate::pos::Pos;

    pub fn board(s: &str) -> Board {
        let s = unindent(s);
        let mut board = Board::empty();

        let lines = s.split("\n");
        for (y, line) in lines.enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos= Pos::graphical(x as u8, y as u8).and_then(|pos| {
                    match c {
                        '1' => Some(pos),
                        _ => None,
                    }
                });
                match pos {
                    Some(pos) => board |= pos.board(),
                    None => (),
                }
            }
        }

        board
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use crate::pos::Pos;
    use std::iter::zip;

    #[test]
    fn iter_enumerates_specified_positions() {
        let target = testutil::board(r"
            _1_0_0_0
            1_0_0_0_
            _0_0_0_0
            0_0_0_0_
            _0_0_1_0
            0_0_0_0_
            _0_1_0_0
            0_0_0_1_
        ");

        let actuals = target.actives();
        let expects = [
            Pos::raw(0),
            Pos::raw(6),
            Pos::raw(13),
            Pos::raw(27),
            Pos::raw(31),
        ];

        for (actual, expect) in zip(actuals, expects) {
            assert_eq!(actual, expect);
        }
    }

    #[test]
    fn iter_enumerates_final_position() {
        let target = testutil::board(r"
            _1_0_0_0
            0_0_0_0_
            _0_0_0_0
            0_0_0_0_
            _0_0_0_0
            0_0_0_0_
            _0_0_0_0
            0_0_0_0_
        ");

        let actuals = target.actives();
        let expects = [ Pos::raw(31) ];

        for (actual, expect) in zip(actuals, expects) {
            assert_eq!(actual, expect);
        }
    }

    #[test]
    fn iter_enumerates_first_position() {
        let target = testutil::board(r"
            _0_0_0_0
            0_0_0_0_
            _0_0_0_0
            0_0_0_0_
            _0_0_0_0
            0_0_0_0_
            _0_0_0_0
            0_0_0_1_
        ");

        let actuals = target.actives();
        let expects = [ Pos::raw(0) ];

        for (actual, expect) in zip(actuals, expects) {
            assert_eq!(actual, expect);
        }
    }
}
