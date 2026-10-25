use eframe::{App, Frame};
use egui::{CentralPanel, Context, Ui};

use checkers::{Board, Game, Move, Player, Pos};

use crate::cell::{Cell, CellKind};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum Mode {
    #[default]
    SelectingMovePiece,
    SelectingDestCell {
        src: Pos,
    },
}

pub struct Checkers {
    mode: Mode,
    game: Game,
}

impl Checkers {
    const ROWS: usize = 8;
    const COLUMNS: usize = 8;

    fn win_text_style() -> egui::TextStyle {
        egui::TextStyle::Name("WinTextStyle".into())
    }

    pub fn new(cc: &egui::Context) -> Checkers {
        let mut style = (*cc.style()).clone();
        style.text_styles.insert(
            Self::win_text_style(),
            egui::FontId::new(80.0, egui::FontFamily::Proportional),
        );
        cc.set_style(style);

        Self {
            mode: Mode::SelectingMovePiece,
            game: Game {
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

    fn render_cell(&mut self, ui: &mut Ui, pos: Pos, moves: &[Move]) {
        let kind = match (pos.is(self.game.blk), pos.is(self.game.red)) {
            (true, false) => CellKind::Blk,
            (false, true) => CellKind::Red,
            (_, _) => CellKind::Empty,
        };
        let selected = match self.mode {
            Mode::SelectingMovePiece => false,
            Mode::SelectingDestCell { src } => src == pos,
        };
        let selectable = match self.mode {
            Mode::SelectingMovePiece => moves.iter().any(|m| m.src == pos),
            Mode::SelectingDestCell { src } => moves.iter().any(|m| m.src == src && m.dst() == pos),
        };
        let cell = Cell::new(kind, pos.is(self.game.king), selected, selectable);

        let resp = cell.render(ui);
        match (self.mode, resp.clicked(), selectable) {
            (Mode::SelectingMovePiece, true, true) => {
                self.mode = Mode::SelectingDestCell { src: pos };
            }
            (Mode::SelectingDestCell { src }, true, true) => {
                // TODO: find suitable move for this moving.
                match moves.iter().find(|m| m.src == src && m.dst() == pos) {
                    Some(m) => {
                        self.game = self.game.apply(m);
                        self.mode = Mode::SelectingMovePiece;
                    }
                    None => (),
                }
            }
            (_, _, _) => (),
        }
    }

    fn render(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            if ui.input(|input| input.pointer.secondary_clicked()) {
                self.mode = Mode::SelectingMovePiece;
            }
            let moves: Vec<Move> = self.game.moves().collect();
            for y in 0..Self::ROWS {
                ui.columns(Self::COLUMNS, |columns| {
                    for (x, column) in columns.iter_mut().enumerate() {
                        match Pos::graphical(x as u8, y as u8) {
                            Some(pos) => self.render_cell(column, pos, &moves),
                            None => self.render_empty_cell(column),
                        }
                    }
                });
                ui.end_row();
            }
            let (color, text) = match self.game.winner() {
                Some(Player::BLK) => (egui::Color32::BLACK, "BLK WIN"),
                Some(Player::RED) => (egui::Color32::RED, "RED WIN"),
                None => (egui::Color32::default(), ""),
            };
            let rect = ui.clip_rect();
            ui.allocate_new_ui(egui::UiBuilder::new().max_rect(rect), |ui| {
                ui.centered_and_justified(|ui| {
                    let label = egui::RichText::new(text)
                        .text_style(Self::win_text_style())
                        .strong();
                    ui.colored_label(color, label);
                });
            });
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cell_frame(ctx: &Context, app: &mut Checkers, pos: Pos, pressed: bool) {
        let pointer = egui::pos2(24.0, 24.0);
        let input = egui::RawInput {
            events: vec![
                egui::Event::PointerMoved(pointer),
                egui::Event::PointerButton {
                    pos: pointer,
                    button: egui::PointerButton::Primary,
                    pressed,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
            ..Default::default()
        };
        let moves: Vec<_> = app.game.moves().collect();
        let _ = ctx.run(input, |ctx| {
            CentralPanel::default().show(ctx, |ui| app.render_cell(ui, pos, &moves));
        });
    }

    #[test]
    fn pointer_click_selects_piece_and_applies_legal_move() {
        let ctx = Context::default();
        let mut app = Checkers::new(&ctx);
        let m = app.game.moves().next().expect("initial position has moves");
        let expected = app.game.apply(&m);
        let moves: Vec<_> = app.game.moves().collect();
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            CentralPanel::default().show(ctx, |ui| app.render_cell(ui, m.src, &moves));
        });
        cell_frame(&ctx, &mut app, m.src, true);
        cell_frame(&ctx, &mut app, m.src, false);
        assert!(matches!(app.mode, Mode::SelectingDestCell { src } if src == m.src));
        cell_frame(&ctx, &mut app, m.dst(), true);
        cell_frame(&ctx, &mut app, m.dst(), false);
        assert_eq!(app.game, expected);
        assert!(matches!(app.mode, Mode::SelectingMovePiece));
    }

    #[test]
    fn rendering_preserves_position_and_secondary_click_cancels_selection() {
        let ctx = Context::default();
        let mut app = Checkers::new(&ctx);
        let before = app.game.clone();
        let src = app.game.moves().next().unwrap().src;
        app.mode = Mode::SelectingDestCell { src };
        let output = ctx.run(egui::RawInput::default(), |ctx| app.render(ctx));
        assert!(!output.shapes.is_empty());
        for pressed in [true, false] {
            let input = egui::RawInput {
                events: vec![egui::Event::PointerButton {
                    pos: egui::pos2(24.0, 24.0),
                    button: egui::PointerButton::Secondary,
                    pressed,
                    modifiers: egui::Modifiers::NONE,
                }],
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| app.render(ctx));
        }
        assert_eq!(app.game, before);
        assert!(matches!(app.mode, Mode::SelectingMovePiece));
    }
}

impl App for Checkers {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.render(ctx)
    }
}
