use derive_more::Not;

#[derive(
    Debug, Default,
    Clone, Copy,
    PartialEq, Eq,
    Not,
)]
pub struct Player(bool);

pub const Red: Player = Player(false);
pub const Blk: Player = Player(true);

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn not() {
        assert_eq!(!Red, Blk);
        assert_eq!(!Blk, Red);
    }
}
