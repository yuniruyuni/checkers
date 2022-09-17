mod app;
mod cell;

use eframe::{run_native, NativeOptions, Theme};
use egui::Vec2;

use crate::app::Checkers;

const WINDOW_WIDTH: f32 = 480.0;
const WINDOW_HEIGHT: f32 = 300.0;

fn main() {
    let mut options = NativeOptions::default();
    options.resizable = false;
    options.initial_window_size = Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT));

    options.default_theme = Theme::Light;

    run_native(
        "checkers",
        options,
        Box::new(|cc| Box::new(Checkers::new(&cc.egui_ctx))),
    );
}
