mod cell;
mod app;

use egui::Vec2;
use eframe::{Theme, NativeOptions, run_native};

use crate::app::Checkers;

const WINDOW_WIDTH: f32 = 480.0;
const WINDOW_HEIGHT: f32 = 300.0;

fn main() {
    let mut options = NativeOptions::default();
    options.resizable = false;
    options.initial_window_size = Some(
        Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)
    );

    options.default_theme = Theme::Light;

    run_native(
        "checkers",
        options,
        Box::new(|_cc| { Box::new(Checkers::new()) }),
    );
}
