#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Déclaration des modules
mod engine;
mod gui;

use crate::gui::CalculatriceApp;
use eframe::egui;
use eframe::egui::IconData;
use std::sync::Arc;

fn main() -> eframe::Result<()> {
    // Chargement sécurisé de l'icône
    let icon = (|| {
        let icon_bytes = include_bytes!("../icons/icon.png");
        let image = image::load_from_memory(icon_bytes).ok()?;
        let rgba8 = image.to_rgba8();
        let (width, height) = rgba8.dimensions();
        Some(IconData {
            rgba: rgba8.into_raw(),
            width,
            height,
        })
    })();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([340.0, 620.0])
            .with_icon(Arc::new(icon.unwrap_or_default()))
            .with_resizable(false),
        ..Default::default()
    };

    println!("Lancement de RustCalc...");

    eframe::run_native(
        "RustCalc",
        native_options,
        Box::new(|_cc| Ok(Box::new(CalculatriceApp::default()))),
    )
}
