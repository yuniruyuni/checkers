use crate::dir::Dir;
use crate::pos::Pos;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Move {
    pub src: Pos,
    pub dir: Dir,
    pub jump: bool,
}

impl Move {
    /// cands() enumerate move candidate for specific position piece.
    pub fn cands(src: Pos, jump: bool) -> [Move; 4] {
        [
            Move {
                src,
                jump,
                dir: Dir::ForwardRight,
            },
            Move {
                src,
                jump,
                dir: Dir::ForwardLeft,
            },
            Move {
                src,
                jump,
                dir: Dir::BackwardLeft,
            },
            Move {
                src,
                jump,
                dir: Dir::BackwardRight,
            },
        ]
    }

    /// dst() return destination position of this move.
    pub fn dst(&self) -> Pos {
        let mut moved = self.dir.apply(self.src.board());
        if self.jump {
            moved = self.dir.apply(moved);
        }
        // it must has a position if this move is valid.
        let v: Vec<Pos> = moved.actives().collect();
        v[0]
    }

    /// mid() returns middle position(=the cell contains a peace captured by jump) of jump move.
    pub fn mid(&self) -> Pos {
        let v: Vec<Pos> = self.dir.apply(self.src.board()).actives().collect();
        v[0]
    }
}
