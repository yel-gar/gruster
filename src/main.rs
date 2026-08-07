mod secret;
mod util;
mod wintypes;

use crate::wintypes::WindowType;
use eframe::epaint::{Direction, FontFamily};
use eframe::{CreationContext, Frame};
use egui::{
    Align, FontData, FontDefinitions, Layout, RichText, Ui, ViewportBuilder, ViewportCommand,
    ViewportId,
};
use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Instant;

struct WindowData {
    id: u32,
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    last_step_time: Instant,
    win_type: WindowType,
}

impl WindowData {
    fn new(id: u32, speed_range: f32) -> Self {
        let spd_rng = -speed_range..speed_range;
        let x = rand::random_range(0.0..1600.0);
        let y = rand::random_range(0.0..800.0);
        let dx = rand::random_range(spd_rng.clone());
        let dy = rand::random_range(spd_rng);
        Self {
            id,
            x,
            y,
            dx,
            dy,
            last_step_time: Instant::now(),
            win_type: WindowType::from_id(id),
        }
    }

    fn step(&mut self) {
        if self.last_step_time.elapsed().as_secs_f32() < 1.0 / 60.0 {
            return;
        }
        self.x += self.dx;
        self.y += self.dy;
        if self.x > 1600.0 {
            self.dx = -self.dx.abs();
        }
        if self.x < 0.0 {
            self.dx = self.dx.abs();
        }
        if self.y > 800.0 {
            self.dy = -self.dy.abs();
        }
        if self.y < 0.0 {
            self.dy = self.dy.abs();
        }
        self.last_step_time = Instant::now();
    }
}

struct App {
    id_counter: u32,
    windows: BTreeMap<u32, WindowData>,
    speed_multiplier: f32,
}

impl App {
    fn new(cc: &CreationContext) -> Self {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "GREENDINGGASTER".to_owned(),
            Arc::new(FontData::from_static(include_bytes!(
                "C:/Windows/Fonts/wingding.ttf"
            ))),
        );
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "GREENDINGGASTER".to_owned());
        fonts
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .insert(0, "GREENDINGGASTER".to_owned());
        cc.egui_ctx.set_fonts(fonts);
        Self {
            id_counter: 0, // TODO: before production MAKE SURE THIS IS ZERO
            windows: BTreeMap::new(),
            speed_multiplier: 1.0,
        }
    }

    fn mitosis(&mut self, source: u32) {
        self.windows.remove(&source);
        self.new_window();
        self.new_window();
    }

    fn new_window(&mut self) -> u32 {
        self.id_counter += 1;
        self.windows.insert(
            self.id_counter,
            WindowData::new(self.id_counter, self.get_speed()),
        );
        self.id_counter
    }

    fn get_speed(&self) -> f32 {
        self.id_counter as f32 * self.speed_multiplier * 0.1
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut Ui, frame: &mut Frame) {
        // root
        ui.label(RichText::new("hello").size(32.0));
        if ui.ctx().input(|i| i.viewport().close_requested()) {
            self.mitosis(0);
            ui.ctx().send_viewport_cmd(ViewportCommand::CancelClose);
            ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
        }

        // clones
        let mut mitosis_targets = Vec::new();
        for (id, window) in self.windows.iter_mut() {
            window.step();
            ui.ctx().show_viewport_immediate(
                ViewportId::from_hash_of(id),
                ViewportBuilder::default()
                    .with_position([window.x, window.y])
                    .with_inner_size([400.0, 300.0])
                    .with_title("Balls"),
                |ui, _| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(50.0);
                        match window.win_type.clone() {
                            WindowType::Root | WindowType::Normal => {
                                ui.label(RichText::new("PLACEHOLDER").size(32.0));
                            }
                            WindowType::Message(msg) => {
                                ui.label(RichText::new("PLACEHOLDER").size(32.0));
                                ui.label(RichText::new(msg).size(32.0));
                            }
                            WindowType::Slowdown(lbl, btn) => {
                                ui.label(RichText::new(lbl).size(32.0));
                                if ui.button(RichText::new(btn).size(32.0)).clicked() {
                                    self.speed_multiplier *= 0.5;
                                    ui.ctx().send_viewport_cmd(ViewportCommand::Close);
                                }
                            }
                            WindowType::Accelerate(lbl, btn) => {
                                ui.label(RichText::new(lbl).size(32.0));
                                if ui.button(RichText::new(btn).size(32.0)).clicked() {
                                    self.speed_multiplier *= 2.0;
                                    ui.ctx().send_viewport_cmd(ViewportCommand::Close);
                                }
                            }
                            WindowType::Prompt => {
                                todo!()
                            }
                            WindowType::Flag => {
                                todo!()
                            }
                        }
                    });

                    if ui.ctx().input(|i| i.viewport().close_requested()) {
                        mitosis_targets.push(window.id);
                    }
                },
            )
        }

        for id in mitosis_targets {
            self.mitosis(id);
        }

        ui.ctx().request_repaint_after_secs(1.0 / 60.0);
    }
}

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Very good appp",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )?;
    Ok(())
}
