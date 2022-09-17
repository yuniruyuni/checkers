use derive_more::Not;

#[derive(Debug, PartialEq, Eq, Not)]
pub struct Piece(bool);

pub const PONE: Piece = Piece(false);
pub const KING: Piece = Piece(true);

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn not() {
        assert_eq!(!PONE, KING);
        assert_eq!(!KING, PONE);
    }
}
