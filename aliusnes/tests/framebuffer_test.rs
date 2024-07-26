use std::{fs, path::Path};

use aliusnes::emu::Emu;
use image::{Rgb, RgbImage};
use pretty_assertions::assert_eq;

include!(concat!(env!("OUT_DIR"), "/krom_test.rs"));

fn compare_to_reference(rom_path: &Path, png_path: &Path) {
    let reference = image::open(png_path).unwrap().into_rgb8();

    let rom = fs::read(rom_path).expect("Couldn't load ROM");
    let ram: Vec<u8> = Vec::new();
    let cart = aliusnes::load_cart(&rom, ram);
    let mut emu = Emu::new(cart);

    emu.run_for_frames(20);

    let mut result = RgbImage::new(256, 224);
    for y in 0..224 {
        for x in 0..256 {
            let pixel = emu.frame()[y * 256 + x];
            result[(x as u32, y as u32)] = Rgb::from(pixel);
        }
    }

    for (px_res, px_ref) in result.pixels().zip(reference.pixels()) {
        assert_eq!(px_res, px_ref);
    }
}
