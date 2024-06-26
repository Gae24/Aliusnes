use crate::emu_state::EmuState;
use aliusnes::cart::Cart;
use eframe::{
    egui::{self, Color32, ColorImage},
    CreationContext,
};

const U5_TO_U8_CONVERSION: f32 = 8.225806;

pub struct App {
    emu_state: EmuState,
    texture: egui::TextureHandle,
}

impl App {
    pub fn new(cc: &CreationContext<'_>, cart: Cart) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self {
            emu_state: EmuState::new(cart),
            texture: cc.egui_ctx.load_texture(
                "Framebuffer",
                egui::ColorImage::new([512, 478], egui::Color32::TRANSPARENT),
                Default::default(),
            ),
        }
    }

    pub fn r_g_b_from_rgb5(value: u16) -> (u8, u8, u8) {
        (
            ((value & 0x1F) as f32 * U5_TO_U8_CONVERSION) as u8,
            ((value >> 5 & 0x1F) as f32 * U5_TO_U8_CONVERSION) as u8,
            ((value >> 10 & 0x1F) as f32 * U5_TO_U8_CONVERSION) as u8,
        )
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::SidePanel::left("my_left_panel").show(ctx, |ui| {
            ui.label("CPU disasm");
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Ok(frame) = self.emu_state.frame_rx.pop() {
                let mut image = ColorImage::new([frame.width, frame.height], Color32::TRANSPARENT);

                for y in 0..image.height() {
                    for x in 0..image.width() {
                        let (r, g, b) = App::r_g_b_from_rgb5(frame.buffer[y * image.width() + x]);
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
    }
}
