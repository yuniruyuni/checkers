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
