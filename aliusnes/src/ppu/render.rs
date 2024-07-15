use crate::ppu::tile::Bpp8;

use super::{
    tile::{BitPlane, Bpp2, Bpp4, TileMapEntry},
    Ppu, WIDTH,
};

#[derive(Clone, Copy)]
enum BackgroundId {
    BG1 = 0,
    BG2,
    BG3,
    BG4,
}

#[derive(Clone, Copy)]
enum Layer {
    Background(BackgroundId, bool),
    Object(u8),
}

use BackgroundId::*;
use Layer::*;

const S0: Layer = Object(0);
const S1: Layer = Object(1);
const S2: Layer = Object(2);
const S3: Layer = Object(3);
const L1: Layer = Background(BG1, false);
const L2: Layer = Background(BG2, false);
const L3: Layer = Background(BG3, false);
const L4: Layer = Background(BG4, false);
const H1: Layer = Background(BG1, true);
const H2: Layer = Background(BG2, true);
const H3: Layer = Background(BG3, true);
const H4: Layer = Background(BG4, true);

impl Ppu {
    pub fn render_scanline(&mut self, screen_y: usize) {
        let fb_line_start = (screen_y - 1) * WIDTH;

        if self.ini_display.screen_brightness() == 0 || self.ini_display.force_blanking() {
            self.frame_buffer[fb_line_start..fb_line_start + self.screen_width].fill(0);
            return;
        }

        let mut bg_data: [[(u16, bool); WIDTH]; 4] = [
            [(0, false); WIDTH],
            [(0, false); WIDTH],
            [(0, false); WIDTH],
            [(0, false); WIDTH],
        ];
        let layers = self.decode_bg_mode(screen_y, &mut bg_data);

        //todo sprite and sub screen rendering

        let fb_line = &mut self.frame_buffer[fb_line_start..fb_line_start + self.screen_width];

        fb_line.fill(self.color.cgram[0]);
        for layer in layers.iter().rev() {
            match layer {
                Background(id, layer_priority) => {
                    if !self.background.backgrounds[*id as usize].enabled_on_main_screen {
                        continue;
                    }
                    for (x, (pixel, priority)) in bg_data[*id as usize].iter().enumerate() {
                        if layer_priority != priority {
                            continue;
                        }
                        if *pixel > 0 {
                            fb_line[x] = *pixel;
                        }
                    }
                }
                Object(_layer_priority) => {}
            }
        }
    }

    fn decode_bg_mode(
        &self,
        screen_y: usize,
        bg_data: &mut [[(u16, bool); WIDTH]; 4],
    ) -> Vec<Layer> {
        match self.background.bg_mode.bg_mode() {
            0 => {
                self.draw_background::<Bpp2, 0>(screen_y, BG1, &mut (bg_data)[BG1 as usize]);
                self.draw_background::<Bpp2, 0>(screen_y, BG2, &mut (bg_data)[BG2 as usize]);
                self.draw_background::<Bpp2, 0>(screen_y, BG3, &mut (bg_data)[BG3 as usize]);
                self.draw_background::<Bpp2, 0>(screen_y, BG4, &mut (bg_data)[BG4 as usize]);
                vec![S3, H1, H2, S2, L1, L2, S1, H3, H4, S0, L3, L4]
            }
            1 => {
                self.draw_background::<Bpp4, 1>(screen_y, BG1, &mut (*bg_data)[BG1 as usize]);
                self.draw_background::<Bpp4, 1>(screen_y, BG2, &mut (*bg_data)[BG2 as usize]);
                self.draw_background::<Bpp2, 1>(screen_y, BG3, &mut (*bg_data)[BG3 as usize]);
                if self.background.bg_mode.bg3_has_priority() {
                    vec![H3, S3, H1, H2, S2, L1, L2, S1, S0, L3]
                } else {
                    vec![S3, H1, H2, S2, L1, L2, S1, H3, S0, L3]
                }
            }
            2 => {
                self.draw_background::<Bpp4, 2>(screen_y, BG1, &mut (*bg_data)[BG1 as usize]);
                self.draw_background::<Bpp4, 2>(screen_y, BG2, &mut (*bg_data)[BG2 as usize]);
                vec![S3, H1, S2, H2, S1, L1, S0, L2]
            }
            3 => {
                self.draw_background::<Bpp8, 3>(screen_y, BG1, &mut (*bg_data)[BG1 as usize]);
                self.draw_background::<Bpp4, 3>(screen_y, BG2, &mut (*bg_data)[BG2 as usize]);
                vec![S3, H1, S2, H2, S1, L1, S0, L2]
            }
            4 => {
                self.draw_background::<Bpp8, 4>(screen_y, BG1, &mut (*bg_data)[BG1 as usize]);
                self.draw_background::<Bpp2, 4>(screen_y, BG2, &mut (*bg_data)[BG2 as usize]);
                vec![S3, H1, S2, H2, S1, L1, S0, L2]
            }
            _ => unimplemented!("mode {}", self.background.bg_mode.bg_mode()),
        }
    }

    fn draw_background<BPP: BitPlane, const BG_MODE: u8>(
        &self,
        screen_y: usize,
        bg_idx: BackgroundId,
        data: &mut [(u16, bool); 256],
    ) where
        [(); BPP::PLANES]:,
    {
        let bg = self.background.backgrounds[bg_idx as usize];
        if !bg.enabled_on_main_screen {
            return;
        }

        let y = screen_y + bg.bg_vofs as usize;
        for (screen_x, pixel_data) in data.iter_mut().enumerate().take(self.screen_width) {
            let x = screen_x + bg.bg_hofs as usize;

            let tile_map_addr = bg.tile_map_addr(x / 8, y / 8);
            let tile = TileMapEntry(self.vram[tile_map_addr]);

            let (col, row) = tile.adjust_coords_to_flipping(x, y);

            let tile_addr = bg.tileset_addr + tile.tile_index() * BPP::WORDS_PER_ROW * 8;
            let planes = self.vram.planes(tile_addr as usize + row);

            let raw_pixel = BPP::pixel(planes, col);
            let pixel = if raw_pixel == 0 {
                0
            } else {
                self.color.pixel_color::<BPP, BG_MODE>(
                    bg_idx as u8,
                    tile.palette_selection(),
                    raw_pixel,
                )
            };

            *pixel_data = (pixel, tile.priority());
        }
    }
}
