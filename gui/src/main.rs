mod app;
mod cell;

use eframe::{run_native, Theme};
use egui::Vec2;

use crate::app::Checkers;

const WINDOW_WIDTH: f32 = 480.0;
const WINDOW_HEIGHT: f32 = 300.0;

fn main() {
    let options = eframe::NativeOptions {
        resizable: false,
        initial_window_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
        default_theme: Theme::Light,
        ..Default::default()
    };

    run_native(
        "checkers",
        options,
        Box::new(|cc| Box::new(Checkers::new(&cc.egui_ctx))),
    );
}
