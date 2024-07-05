use std::{env, fs, path::Path};

mod app;
mod emu_state;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = env::args().collect();

    if !args.is_empty() {
        let rom_name = &args[1];
        let rom_path = Path::new(rom_name);

        if let Some(extension) = rom_path.extension().and_then(|s| s.to_str()) {
            if extension != "sfc" && extension != "smc" {
                println!("File not supported");
                return;
            }
        }

        let rom = fs::read(rom_path).expect("Couldn't load ROM");
        let ram: Vec<u8> = Vec::new();

        let cart = aliusnes::load_cart(&rom, ram);

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
    } else {
        println!("Rom file not provided");
    }
}
