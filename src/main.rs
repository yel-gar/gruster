mod resources;
mod secret;
mod util;
mod wintypes;

use crate::resources::{MUSIC, MYSTERY_MAN_PNG, TALK};
use crate::util::{color_from_lerp_f, decrypt_flag};
use crate::wintypes::WindowType;
use eframe::epaint::FontFamily;
use eframe::{CreationContext, Frame};
use egui::{
    Context, FontData, FontDefinitions, Image, RichText, Ui, ViewportBuilder, ViewportCommand,
    ViewportId,
};
use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player, Source};
use std::collections::BTreeMap;
use std::io::Cursor;
use std::sync::Arc;
use std::time::{Duration, Instant};

struct WindowData {
    id: u32,
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    last_step_time: Instant,
    initialized_at: Instant,
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
            initialized_at: Instant::now(),
            win_type: WindowType::from_id(id),
        }
    }

    fn step(&mut self) {
        if self.last_step_time.elapsed().as_secs_f32() < 1.0 / 60.0 {
            return;
        }
        self.x += self.dx;
        self.y += self.dy;
        if self.x > 1500.0 {
            self.dx = -self.dx.abs();
        }
        if self.x < 0.0 {
            self.dx = self.dx.abs();
        }
        if self.y > 700.0 {
            self.dy = -self.dy.abs();
        }
        if self.y < 0.0 {
            self.dy = self.dy.abs();
        }
        self.last_step_time = Instant::now();
    }

    fn current_text(&self) -> String {
        match &self.win_type {
            WindowType::Message(s) | WindowType::Accelerate(s, _) | WindowType::Slowdown(s, _) => {
                let progress = (self.initialized_at.elapsed().as_secs_f32() / 2.0).min(1.0);
                if progress >= 1.0 {
                    return s.clone();
                }
                let take_below = (s.chars().count() as f32 * progress).floor() as usize;
                s.chars().take(take_below).collect::<String>()
            }
            _ => "".to_string(),
        }
    }
}

struct App {
    old_id_counter: u32,
    id_counter: u32,
    windows: BTreeMap<u32, WindowData>,
    speed_multiplier: f32,
    password: String,
    sink: MixerDeviceSink,
    player: Player,
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

        egui_extras::install_image_loaders(&cc.egui_ctx);

        let sink = DeviceSinkBuilder::open_default_sink().expect("Failed to open audio device");
        let player = Player::connect_new(&sink.mixer());
        let source = Decoder::new_mp3(Cursor::new(MUSIC))
            .expect("Failed to load audio source")
            .repeat_infinite();
        player.append(source);
        player.set_volume(0.0);

        Self {
            old_id_counter: 0,
            id_counter: 0, // TODO: before production MAKE SURE THIS IS ZERO
            windows: BTreeMap::new(),
            speed_multiplier: 1.0,
            password: String::new(),
            player,
            sink,
        }
    }

    fn mitosis(&mut self, source: u32) {
        self.windows.remove(&source);
        self.new_window();
        self.new_window();
    }

    fn new_window(&mut self) -> u32 {
        self.id_counter += 1;
        let window = WindowData::new(self.id_counter, self.get_speed());
        let win_type = window.win_type.clone();
        self.windows.insert(self.id_counter, window);

        if win_type != WindowType::Normal {
            let mut decoder =
                Decoder::new_mp3(Cursor::new(TALK)).expect("Failed to load audio source");
            let _ = decoder.try_seek(Duration::from_secs_f32(rand::random_range(0.0..1.9)));
            self.sink
                .mixer()
                .add(decoder.take_duration(Duration::from_secs(2)))
        }
        self.id_counter
    }

    fn get_speed(&self) -> f32 {
        self.id_counter as f32 * self.speed_multiplier * 0.1
    }
}

impl eframe::App for App {
    fn logic(&mut self, _ctx: &Context, _frame: &mut Frame) {
        if self.old_id_counter == self.id_counter {
            return;
        }
        self.old_id_counter = self.id_counter;
        self.player.set_volume(self.id_counter as f32 / 100.0);
    }

    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        // root
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            ui.add(
                Image::new(MYSTERY_MAN_PNG)
                    .max_height(160.0)
                    .tint(color_from_lerp_f((self.id_counter as f32 / 666.0).min(1.0))),
            );
        });
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
                    .with_resizable(false)
                    .with_title("???"),
                |ui, _| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(50.0);
                        match window.win_type.clone() {
                            WindowType::Root | WindowType::Normal => {
                                ui.add(
                                    Image::new(MYSTERY_MAN_PNG)
                                        .max_height(160.0)
                                        .tint(color_from_lerp_f((*id as f32 / 666.0).min(1.0))),
                                );
                            }
                            WindowType::Message(_) => {
                                ui.label(RichText::new(window.current_text()).size(25.0));
                                ui.add(
                                    Image::new(MYSTERY_MAN_PNG)
                                        .max_height(160.0)
                                        .tint(color_from_lerp_f((*id as f32 / 666.0).min(1.0))),
                                );
                            }
                            WindowType::Slowdown(_, btn) => {
                                ui.label(RichText::new(window.current_text()).size(28.0));
                                if ui.button(RichText::new(btn).size(32.0)).clicked() {
                                    self.speed_multiplier *= 0.5;
                                    ui.ctx().send_viewport_cmd(ViewportCommand::Close);
                                }
                            }
                            WindowType::Accelerate(_, btn) => {
                                ui.label(RichText::new(window.current_text()).size(28.0));
                                if ui.button(RichText::new(btn).size(32.0)).clicked() {
                                    self.speed_multiplier *= 2.0;
                                    ui.ctx().send_viewport_cmd(ViewportCommand::Close);
                                }
                            }
                            WindowType::Prompt => {
                                ui.label(RichText::new("PASSWORD").size(32.0));
                                ui.text_edit_singleline(&mut self.password);
                            }
                            WindowType::Flag => {
                                ui.label(RichText::new("CONGRATULATIONS (OR NOT)").size(16.0));
                                ui.add_space(50.0);
                                ui.label(RichText::new(decrypt_flag(&self.password)).size(16.0));
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
    let opts = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native("???", opts, Box::new(|cc| Ok(Box::new(App::new(cc)))))?;
    Ok(())
}
