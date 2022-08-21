use derive_more::{
    BitAnd, BitAndAssign,
    BitOr, BitOrAssign,
    BitXor, BitXorAssign,
    Shl, ShlAssign,
    Shr, ShrAssign,
    Not,
};

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
}

#[cfg(test)]
pub mod tests {
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
