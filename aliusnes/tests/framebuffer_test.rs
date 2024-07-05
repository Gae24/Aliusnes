use std::{fs, path::Path};

use aliusnes::emu::Emu;
use image::{Rgba, RgbaImage};

const U5_TO_U8_CONVERSION: f32 = 8.225806;

include!(concat!(env!("OUT_DIR"), "/krom_test.rs"));

fn compare_to_reference(rom_path: &Path, png_path: &Path) {
    let reference = image::open(png_path).unwrap().into_rgba8();

    let rom = fs::read(rom_path).expect("Couldn't load ROM");
    let ram: Vec<u8> = Vec::new();
    let cart = aliusnes::load_cart(&rom, ram);
    let mut emu = Emu::new(cart);

    emu.run_for_frames(6);

    let mut result = RgbaImage::new(256, 224);
    for y in 0..224 {
        for x in 0..256 {
            let pixel = emu.frame()[y * 256 + x];
            result[(x as u32, y as u32)] = Rgba::from(rgba_from_rgb5(pixel));
        }
    }

    if result != reference {
        panic!("Frame does not match reference");
    }
}

fn rgba_from_rgb5(value: u16) -> [u8; 4] {
    let r = ((value & 0x1F) as f32 * U5_TO_U8_CONVERSION) as u8;
    let g = ((value >> 5 & 0x1F) as f32 * U5_TO_U8_CONVERSION) as u8;
    let b = ((value >> 10 & 0x1F) as f32 * U5_TO_U8_CONVERSION) as u8;
    [r, g, b, 255]
}
