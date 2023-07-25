#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::sync::{Arc, RwLock};

use eframe::egui::{self, window::ViewportBuilder};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    // Our application state:
    let mut name = "Arthur".to_owned();
    let mut age = 42;

    let mut window1_embedded = Arc::new(RwLock::new(true));
    let mut window2_embedded = Arc::new(RwLock::new(true));

    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!(
                "Current rendering window: {}",
                ctx.current_rendering_viewport()
            ));
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                age += 1;
            }
            ui.label(format!("Hello '{name}', age {age}"));
            let mut is_desktop = ctx.is_desktop();
            ui.checkbox(&mut is_desktop, "Is Desktop");
            ctx.set_desktop(is_desktop);

            ctx.create_viewport(
                ViewportBuilder::default()
                    .with_inner_size((50, 30))
                    .with_decorations(false)
                    .with_transparent(true)
                    .with_resizable(false),
                |ctx, _, _| {
                    let size = egui::Rect::from_min_size(
                        egui::Pos2::new(0.0, 0.0),
                        egui::Vec2::new(50.0, 50.0),
                    );
                    let mut ui = egui::Ui::new(
                        ctx.clone(),
                        egui::LayerId::background(),
                        "Viewport Popup".into(),
                        size,
                        size,
                    );
                    egui::Frame::popup(&ctx.style()).show(&mut ui, |ui| {
                        ui.label("Popup");
                    });
                },
            );

            let clone = window1_embedded.clone();
            let embedded = *window1_embedded.read().unwrap();
            egui::CollapsingHeader::new("Show Test1").show(ui, |ui| {
                egui::Window::new("Test1").embedded(embedded).show(
                    ctx,
                    move |ui, id, parent_id| {
                        if ui
                            .checkbox(&mut *clone.write().unwrap(), "Should embedd?")
                            .clicked()
                        {
                            ui.ctx().request_repaint_viewport(parent_id);
                        }
                        let ctx = ui.ctx().clone();
                        ui.label(format!(
                            "Current rendering window: {}",
                            ctx.current_rendering_viewport()
                        ));
                        if ui.button("Drag").is_pointer_button_down_on() {
                            ctx.viewport_command(id, egui::window::ViewportCommand::Drag)
                        }
                    },
                );
            });
            let clone = window2_embedded.clone();
            let embedded = *window2_embedded.read().unwrap();
            egui::CollapsingHeader::new("Shout Test2").show(ui, |ui| {
                egui::Window::new("Test2").embedded(embedded).show(
                    ctx,
                    move |ui, id, parent_id| {
                        if ui
                            .checkbox(&mut *clone.write().unwrap(), "Should embedd?")
                            .clicked()
                        {
                            ui.ctx().request_repaint_viewport(parent_id);
                        }
                        let ctx = ui.ctx().clone();
                        ui.label(format!(
                            "Current rendering window: {}",
                            ctx.current_rendering_viewport()
                        ));

                        if ui.button("Drag").is_pointer_button_down_on() {
                            ctx.viewport_command(id, egui::window::ViewportCommand::Drag)
                        }
                    },
                );
            });
        });
    })
}
