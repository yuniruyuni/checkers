use egui::{Context, CentralPanel, Ui};
use eframe::{App, Frame};

use checkers::{Game, Board, Player, Pos, Move};

use crate::cell::{CellKind, Cell};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum Mode {
    #[default]
    SelectingMovePiece,
    SelectingDestCell{src: Pos},
}

pub struct Checkers {
    mode: Mode,
    game: Game,
}

impl Checkers {
    const ROWS: usize = 8;
    const COLUMNS: usize = 8;

    pub fn new() -> Checkers {
        Self{
            mode: Mode::SelectingMovePiece,
            game: Game{
                side: Player::BLK,
                jumping: None,
                blk: Board::new(0b0000_0000_0000_0000_0000_0000_1111_1111),
                red: Board::new(0b1111_1111_1111_0000_0010_0000_0000_0000),
                king: Board::new(0b0000_1000_0000_0000_0000_0000_0000_0000),
            },
        }
    }

    fn render_empty_cell(&mut self, ui: &mut Ui) {
        let cell = Cell::new(CellKind::Empty, false, false, false);
        cell.render(ui);
    }

    fn render_cell(&mut self, ui: &mut Ui, pos: Pos, moves: &Vec<Move>) {
        let kind = match (pos.is(self.game.blk), pos.is(self.game.red)) {
            (true, false) => CellKind::Blk,
            (false, true) => CellKind::Red,
            (_, _) => CellKind::Empty,
        };
        let selected = match self.mode {
            Mode::SelectingMovePiece => false,
            Mode::SelectingDestCell{ src } => src == pos,
        };
        let selectable = match self.mode {
            Mode::SelectingMovePiece => moves.iter().find(|m| m.src == pos).is_some(),
            Mode::SelectingDestCell { src } =>
                moves.iter().find(|m| m.src == src && m.dst() == pos).is_some(),
        };
        let cell = Cell::new(kind, pos.is(self.game.king), selected, selectable);

        let resp = cell.render(ui);
        if resp.clicked() {
            match self.mode {
                Mode::SelectingMovePiece => {
                    self.mode = Mode::SelectingDestCell { src: pos };
                },
                Mode::SelectingDestCell{ src } => {
                    // TODO: find suitable move for this moving.
                    match moves.iter().find(|m| m.src == src && m.dst() == pos) {
                        Some(m) => {
                            self.game = self.game.apply(m);
                        },
                        None => (),
                    }
                    self.mode = Mode::SelectingMovePiece;
                },
            }
        }
    }

    fn render(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let moves: Vec<Move> = self.game.moves().collect();
            for y in 0..Self::ROWS {
                ui.columns(Self::COLUMNS, |columns| {
                    for x in 0..Self::COLUMNS {
                        match Pos::graphical(x as u8, y as u8) {
                            Some(pos) => self.render_cell(&mut columns[x], pos, &moves),
                            None => self.render_empty_cell(&mut columns[x]),
                        }
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
