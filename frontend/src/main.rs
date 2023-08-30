use std::{env, fs::File, io::Read, path::Path};

use aliusnes::run_emu;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = env::args().collect();
    let rom_name = &args[0];
    let rom_path = Path::new(rom_name);

    if let Some(extension) = rom_path.extension().and_then(|s| s.to_str()) {
        if extension != "sfc" {
            println!("File not supported");
            return;
        }
    }

    let mut rom_file = File::open(rom_path).expect("Couldn't load ROM");

    let mut rom: Vec<u8> = Vec::new();
    let ram: Vec<u8> = Vec::new();

    rom_file.read_to_end(&mut rom).expect("Couldn't read ROM");

    run_emu(rom, ram);
}
