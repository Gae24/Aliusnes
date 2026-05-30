use aliusnes::cart::Cart;
use eframe::CreationContext;
use eframe::egui::{self, Color32, ColorImage};

use crate::emu_state::{EmuState, Message};

pub struct App {
    emu_state: EmuState,
    playing: bool,
    texture: egui::TextureHandle,
}

impl App {
    pub fn new(cc: &CreationContext<'_>, cart: Cart) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self {
            emu_state: EmuState::new(cart),
            playing: true,
            texture: cc.egui_ctx.load_texture(
                "Framebuffer",
                egui::ColorImage::filled([512, 478], egui::Color32::TRANSPARENT),
                Default::default(),
            ),
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _: &mut eframe::Frame) {
        egui::Panel::left("my_left_panel").show_inside(ui, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                if ui.button("Step").clicked() {
                    self.emu_state.send_message(Message::Step);
                }
                if ui
                    .button(if self.playing { "Pause" } else { "Resume" })
                    .clicked()
                {
                    if self.playing {
                        self.emu_state.send_message(Message::Pause);
                    } else {
                        self.emu_state.send_message(Message::Play);
                    }
                    self.playing = !self.playing;
                }
            });
            ui.label("CPU disasm");
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            if let Ok(frame) = self.emu_state.frame_rx.pop() {
                let mut image =
                    ColorImage::filled([frame.width, frame.height], Color32::TRANSPARENT);

                for y in 0..image.height() {
                    for x in 0..image.width() {
                        let [r, g, b] = frame.buffer[y * image.width() + x];
                        image[(x, y)] = Color32::from_rgb(r, g, b);
                    }
                }
                self.texture.set(image, egui::TextureOptions::default());
            };
            let size = egui::Vec2::new(512.0, 478.0);
            let (whole_rect, _) =
                ui.allocate_exact_size(size, egui::Sense::focusable_noninteractive());
            egui::Image::new((self.texture.id(), self.texture.size_vec2()))
                .paint_at(ui, whole_rect);
        });

        ui.request_repaint();
    }
}
