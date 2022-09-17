use std::collections::HashSet;

use crate::game::Game;

/// History is a sequence of game states.
#[derive(Default)]
pub struct History {
    seq: Vec<Game>,
    set: HashSet<Game>,
}

impl History {
    pub fn push(&mut self, g: Game) {
        self.seq.push(g.clone());
        self.set.insert(g);
    }

    pub fn last(&self) -> Option<&Game> {
        self.seq.last()
    }

    pub fn pop(&mut self) -> Option<Game> {
        self.seq.pop().map(|g| { self.set.remove(&g); g })
    }

    pub fn contains(&self, g: &Game) -> bool {
        self.set.contains(g)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;
    use crate::game::Game;
    use crate::player::Player;
    use pretty_assertions::assert_eq;

    #[test]
    fn push_pop_identity() {
        let g = Game {
            side: Player::BLK,
            jumping: None,
            red: Board::new(0b0000_0001_0000_0000_0001_0010_0000_0100),
            blk: Board::new(0b0100_0000_0000_0010_0000_0000_0000_0010),
            king: Board::new(0b0000_0001_0000_0000_0001_0000_0100_0000),
        };
        let expected = Some(g.clone());

        let mut h = History::default();
        h.push(g);
        let actual = h.pop();

        assert_eq!(expected, actual);
    }

    #[test]
    fn pushed_contains_identity() {
        let g1 = Game {
            side: Player::BLK,
            jumping: None,
            red: Board::new(0b0000_0001_0000_0000_0001_0010_0000_0100),
            blk: Board::new(0b0100_0000_0000_0010_0000_0000_0000_0010),
            king: Board::new(0b0000_0001_0000_0000_0001_0000_0100_0000),
        };
        let g2 = Game {
            side: Player::BLK,
            jumping: None,
            red: Board::new(0b0100_0000_0000_0010_0000_0000_0000_0010),
            blk: Board::new(0b0000_0001_0000_0000_0001_0010_0000_0100),
            king: Board::new(0b0000_0001_0000_0000_0001_0000_0100_0000),
        };

        let mut h = History::default();
        h.push(g1.clone());
        h.push(g2.clone());

        assert_eq!(true, h.contains(&g1));
        assert_eq!(true, h.contains(&g2));
    }
}
