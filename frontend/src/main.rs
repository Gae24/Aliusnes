use std::{
    env,
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::Path,
};

use aliusnes::run_emu;

fn main() {
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
    let mut rom_len = rom_file
        .metadata()
        .expect("Couldn't get ROM metadata")
        .len();
    if rom_len & 0x200 != 0 {
        rom_len -= 0x200;
        rom_file
            .seek(SeekFrom::Start(0x200))
            .expect("Couldn't seek ROM");
    }

    let mut rom: Vec<u8> = Vec::new();
    let ram: Vec<u8> = Vec::new();

    rom_file.read_exact(&mut rom).expect("Couldn't read ROM");

    run_emu(rom, ram);
}
