use derive_more::Not;

#[derive(
    Debug,
    PartialEq, Eq,
    Not,
)]
pub struct Piece(bool);

pub const Pone: Piece = Piece(false);
pub const King: Piece = Piece(true);

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn not() {
        assert_eq!(!Pone, King);
        assert_eq!(!King, Pone);
    }
}
