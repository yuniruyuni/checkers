use derive_more::Not;

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, Not)]
pub struct Player(bool);

impl Player {
    pub const RED: Player = Player(false);
    pub const BLK: Player = Player(true);
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn not() {
        assert_eq!(!Player::RED, Player::BLK);
        assert_eq!(!Player::BLK, Player::RED);
    }
}
