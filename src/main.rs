mod app;
mod calculator;
mod ui;

use app::CalcsPlus;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 650.0])
            .with_min_inner_size([350.0, 500.0])
            .with_max_inner_size([500.0, 900.0])
            .with_title("CalcsPlus - Advanced Calculator")
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "CalcsPlus",
        native_options,
        Box::new(|cc| Ok(Box::new(CalcsPlus::new(cc)))),
    )
}
