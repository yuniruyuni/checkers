mod app;
mod cell;

use eframe::run_native;
use egui::Vec2;

use crate::app::Checkers;

const WINDOW_WIDTH: f32 = 480.0;
const WINDOW_HEIGHT: f32 = 300.0;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(false)
            .with_inner_size(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
        ..Default::default()
    };

    run_native(
        "checkers",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_theme(egui::Theme::Light);
            Ok(Box::new(Checkers::new(&cc.egui_ctx)))
        }),
    )
}
