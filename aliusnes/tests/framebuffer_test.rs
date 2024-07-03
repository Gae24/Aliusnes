use std::path::Path;

include!(concat!(env!("OUT_DIR"), "/krom_test.rs"));

fn compare_to_reference(rom_path: &Path, png_path: &Path) {
    assert_eq!(rom_path, png_path);
}
