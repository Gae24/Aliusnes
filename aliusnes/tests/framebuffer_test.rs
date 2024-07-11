use std::{fs, path::Path};

use aliusnes::emu::Emu;
use image::{Rgb, RgbImage};

include!(concat!(env!("OUT_DIR"), "/krom_test.rs"));

fn compare_to_reference(rom_path: &Path, png_path: &Path) {
    let reference = image::open(png_path).unwrap().into_rgb8();

    let rom = fs::read(rom_path).expect("Couldn't load ROM");
    let ram: Vec<u8> = Vec::new();
    let cart = aliusnes::load_cart(&rom, ram);
    let mut emu = Emu::new(cart);

    emu.run_for_frames(6);

    let mut result = RgbImage::new(256, 224);
    for y in 0..224 {
        for x in 0..256 {
            let pixel = emu.frame()[y * 256 + x];
            result[(x as u32, y as u32)] = Rgb::from(rgb8_from_rgb5(pixel));
        }
    }

    if result != reference {
        panic!("Frame does not match reference");
    }
}

fn rgb8_from_rgb5(value: u16) -> [u8; 3] {
    let r = FIVEBIT_TO_EIGHTBIT_LUT[0xF][usize::from(value & 0x1F)];
    let g = FIVEBIT_TO_EIGHTBIT_LUT[0xF][usize::from(value >> 5 & 0x1F)];
    let b = FIVEBIT_TO_EIGHTBIT_LUT[0xF][usize::from(value >> 10 & 0x1F)];
    [r, g, b]
}

const FIVEBIT_TO_EIGHTBIT_LUT: [[u8; 0x20]; 0x10] = [
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ],
    [
        0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 12, 12, 13, 13, 14,
        14, 15, 15, 16, 17,
    ],
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 24, 25, 26,
        27, 28, 29, 30, 31, 32, 34,
    ],
    [
        0, 1, 3, 4, 6, 8, 9, 11, 13, 14, 16, 18, 19, 21, 23, 24, 26, 27, 29, 31, 32, 34, 36, 37,
        39, 41, 42, 44, 46, 47, 49, 51,
    ],
    [
        0, 2, 4, 6, 8, 10, 13, 15, 17, 19, 21, 24, 26, 28, 30, 32, 34, 37, 39, 41, 43, 45, 48, 50,
        52, 54, 56, 59, 61, 63, 65, 68,
    ],
    [
        0, 2, 5, 8, 10, 13, 16, 19, 21, 24, 27, 30, 32, 35, 38, 41, 43, 46, 49, 52, 54, 57, 60, 63,
        65, 68, 71, 74, 76, 79, 82, 85,
    ],
    [
        0, 3, 6, 9, 12, 16, 19, 22, 26, 29, 32, 36, 39, 42, 46, 49, 52, 55, 59, 62, 65, 68, 72, 75,
        78, 82, 85, 88, 92, 95, 98, 102,
    ],
    [
        0, 3, 7, 11, 14, 19, 22, 26, 30, 34, 38, 42, 45, 49, 53, 57, 61, 64, 69, 72, 76, 80, 84,
        88, 91, 95, 99, 103, 107, 111, 114, 119,
    ],
    [
        0, 4, 8, 12, 17, 21, 26, 30, 34, 39, 43, 48, 52, 56, 61, 65, 69, 74, 78, 83, 87, 91, 96,
        100, 105, 109, 113, 118, 122, 126, 131, 136,
    ],
    [
        0, 4, 9, 14, 19, 24, 29, 34, 39, 44, 49, 54, 58, 63, 69, 73, 78, 83, 88, 93, 98, 103, 108,
        113, 118, 123, 127, 133, 138, 142, 147, 153,
    ],
    [
        0, 5, 10, 16, 21, 27, 32, 38, 43, 49, 54, 60, 65, 70, 76, 82, 87, 92, 98, 104, 109, 114,
        120, 126, 131, 136, 142, 148, 153, 158, 164, 170,
    ],
    [
        0, 5, 11, 17, 23, 30, 35, 41, 47, 54, 60, 66, 71, 77, 84, 90, 96, 101, 108, 114, 120, 126,
        132, 138, 144, 150, 156, 162, 168, 174, 180, 187,
    ],
    [
        0, 6, 12, 19, 25, 32, 39, 45, 52, 59, 65, 72, 78, 84, 92, 98, 104, 111, 118, 124, 131, 137,
        144, 151, 157, 164, 170, 177, 184, 190, 196, 204,
    ],
    [
        0, 6, 13, 20, 27, 35, 42, 49, 56, 64, 71, 78, 84, 91, 99, 106, 113, 120, 128, 135, 142,
        149, 156, 163, 170, 177, 184, 192, 199, 206, 213, 221,
    ],
    [
        0, 7, 14, 22, 29, 38, 45, 53, 60, 69, 76, 84, 91, 98, 107, 114, 122, 129, 138, 145, 153,
        160, 168, 176, 183, 191, 198, 207, 214, 222, 229, 238,
    ],
    [
        0, 8, 16, 24, 33, 41, 49, 57, 66, 74, 82, 90, 99, 107, 115, 123, 132, 140, 148, 156, 165,
        173, 181, 189, 198, 205, 214, 222, 230, 239, 247, 255,
    ],
];
