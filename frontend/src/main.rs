use std::{env, fs, path::Path};

use aliusnes::emu::Emu;

mod app;
mod emu_state;

fn parse_rom(path: &str) -> Option<&Path> {
    let rom_path = Path::new(path);

    if let Some(extension) = rom_path.extension().and_then(|s| s.to_str()) {
        if extension != "sfc" && extension != "smc" {
            return None;
        }
    }
    Some(rom_path)
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        println!("Invalid args");
        return;
    }

    let mut headless = false;
    let mut rom_path: Option<&Path> = None;
    for arg in args.iter() {
        match arg.as_str() {
            "--headless" => headless = true,
            _ => rom_path = parse_rom(arg),
        }
    }

    if rom_path.is_none() {
        println!("Invalid path");
        return;
    }
    let rom = fs::read(rom_path.unwrap()).expect("Couldn't load ROM");
    let ram: Vec<u8> = Vec::new();

    let cart = aliusnes::load_cart(&rom, ram);

    if headless {
        let mut emu = Emu::new(cart);
        loop {
            emu.step();
        }
    } else {
        let native_options = eframe::NativeOptions {
            renderer: eframe::Renderer::Wgpu,
            ..Default::default()
        };

        eframe::run_native(
            "Aliusnes",
            native_options,
            Box::new(|cc| Ok(Box::new(app::App::new(cc, cart)))),
        )
        .unwrap();
    }
}
