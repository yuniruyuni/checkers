use egui::{Context, CentralPanel};
use eframe::{App, Frame};

use checkers::{Game, Board, Player, Pos, Move};

use crate::cell::{CellKind, Cell};

pub struct Checkers {
    game: Game,
}

impl Checkers {
    const ROWS: usize = 8;
    const COLUMNS: usize = 8;

    pub fn new() -> Checkers {
        Self{
            game: Game{
                side: Player::BLK,
                blk: Board::new(0b0000_0000_0000_0000_0000_0010_1111_1111),
                red: Board::new(0b1111_1111_1111_0000_0010_0000_0000_0000),
                king: Board::new(0b0000_1000_0000_0000_0000_0010_0000_0000),
            },
        }
    }

    fn render(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let moves: Vec<Move> = self.game.moves().collect();
            for y in 0..Self::ROWS {
                ui.columns(Self::COLUMNS, |columns| {
                    for x in 0..Self::COLUMNS {
                        let conds =
                            Pos::graphical(x as u8, y as u8).map(|pos| (
                                pos.is(self.game.blk),
                                pos.is(self.game.red),
                                pos.is(self.game.king),
                                moves.iter().find(|m| m.src == pos).is_some(),
                            ));

                        let cell = match conds {
                            Some((true, false, king, selectable)) =>
                                Cell::new(CellKind::Blk, king, selectable),
                            Some((false, true, king, selectable)) =>
                                Cell::new(CellKind::Red, king, selectable),
                            Some(_) | None =>
                                Cell::new(CellKind::Empty, false, false),
                        };

                        cell.render(&mut columns[x]);
                    }
                });
                ui.end_row();
            }
        });
    }
}

impl App for Checkers {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.render(ctx, frame)
    }
}
