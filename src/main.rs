mod app;
mod cycle;
mod exam;
mod fonts;
mod frequency;
mod icons;
mod notification;
mod pomodoro;
mod rogue;
mod storage;
mod theme;
mod widgets;

use app::FernDoroApp;
use eframe::egui::Vec2;

fn main() -> eframe::Result<()> {
    let mut native_options = eframe::NativeOptions::default();
    
    native_options.viewport = egui::ViewportBuilder::default()
        .with_inner_size(Vec2::new(1040.0, 750.0))
        .with_min_inner_size(Vec2::new(880.0, 640.0))
        .with_title("FernDoro - Material You Study & Rogue-Pomodoro Suite");

    eframe::run_native(
        "FernDoro",
        native_options,
        Box::new(|cc| {
            fonts::setup_fonts(&cc.egui_ctx);
            Ok(Box::new(FernDoroApp::new(cc)))
        }),
    )
}
