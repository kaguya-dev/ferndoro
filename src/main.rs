mod app;
mod cycle;
mod frequency;
mod notification;
mod pomodoro;
mod storage;
mod theme;
mod widgets;

use app::FernDoroApp;
use eframe::egui::Vec2;

fn main() -> eframe::Result<()> {
    let mut native_options = eframe::NativeOptions::default();
    
    native_options.viewport = egui::ViewportBuilder::default()
        .with_inner_size(Vec2::new(980.0, 720.0))
        .with_min_inner_size(Vec2::new(820.0, 600.0))
        .with_title("FernDoro - High Contrast Focus & Study Suite");

    eframe::run_native(
        "FernDoro",
        native_options,
        Box::new(|cc| Ok(Box::new(FernDoroApp::new(cc)))),
    )
}
