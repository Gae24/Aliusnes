use std::{env, fs, path::Path};

use aliusnes::run_emu;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = env::args().collect();
    let rom_name = &args[1];
    let rom_path = Path::new(rom_name);

    if let Some(extension) = rom_path.extension().and_then(|s| s.to_str()) {
        if extension != "sfc" {
            println!("File not supported");
            return;
        }
    }

    let rom = fs::read(rom_path).expect("Couldn't load ROM");
    let ram: Vec<u8> = Vec::new();

    run_emu(&rom, ram);
}
