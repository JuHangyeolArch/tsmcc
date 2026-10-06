use tsmcc;

use std::error::Error;
use std::fs;
use std::process;
use std::env;

mod ui;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(
                    &include_bytes!("../assets/a.png")[..],
                )
                .expect("Failed to load icon"),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "TSMCC",
        native_options,
        Box::new(|cc| Ok(Box::new(ui::MainMenu::new(cc)))),
    )
}
