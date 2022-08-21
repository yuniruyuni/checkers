// use checkers::{Pos, Board, Player};

use egui::{Ui, Color32, vec2, Sense, Shape, pos2, Pos2};

#[derive(Debug, Clone, Copy, Default)]
pub enum CellKind {
    #[default]
    Empty,
    Red,
    Blk,
}

impl CellKind {
    fn hovering(self) -> Color32 {
        match self {
            CellKind::Empty => Color32::LIGHT_GREEN,
            CellKind::Red => Color32::LIGHT_RED,
            CellKind::Blk => Color32::LIGHT_RED,
        }
    }

    fn shape_fill(self) -> Color32 {
        match self {
            CellKind::Empty => Color32::default(),
            CellKind::Red => Color32::LIGHT_RED,
            CellKind::Blk => Color32::BLACK,
        }
    }

    fn shape_edge(self) -> Color32 {
        match self {
            CellKind::Empty => Color32::default(),
            CellKind::Red => Color32::WHITE,
            CellKind::Blk => Color32::WHITE,
        }
    }
}

pub struct Cell {
    kind: CellKind,
    king: bool,
    selectable: bool,
}

const CELL_SIZE: f32 = 32.0;

impl Cell {
    pub fn new(kind: CellKind, king: bool, selectable: bool) -> Cell {
        Self{kind, king, selectable}
    }

    pub fn render(&self, ui: &mut Ui) {
        let (resp, painter) = ui.allocate_painter(
            vec2(CELL_SIZE, CELL_SIZE), Sense::click(),
        );

        let stroke = (1.0, Color32::BLACK);
        let background = if resp.hovered() && self.selectable {
            self.kind.hovering()
        } else {
            Color32::default()
        };
        painter.rect(resp.rect, 0.0, background, stroke);

        painter.circle(
            resp.rect.center(),
            14.0,
            self.kind.shape_fill(),
            (1.0, self.kind.shape_edge()),
        );

        if self.king {
        painter.add(Shape::convex_polygon(
            star(resp.rect.center(), 7.0, 12.0).collect(),
            Color32::default(),
            (1.0, self.kind.shape_edge()),
        ));
        }
    }
}

fn star(c: Pos2, ir: f32, er: f32) -> impl Iterator<Item = Pos2> {
    let n = 5 * 2;
    let frac_n = 1.0 / (n as f32);
    let offset = std::f32::consts::TAU / 4.0;

    (0..n).map(move |i|{
        let r = if i % 2 == 0 { ir } else { er };
        let rad = std::f32::consts::TAU * frac_n * (i as f32) + offset;

        let x = r * f32::cos(rad) + c.x;
        let y = r * f32::sin(rad) + c.y;
        pos2(x, y)
    })
}
