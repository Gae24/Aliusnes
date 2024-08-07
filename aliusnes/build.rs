use heck::ToSnakeCase;
use std::{
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=tests");
    generate_tomharte_65816_test();

    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let base_path = root_dir.join("tests/krom/");
    let mut file = File::create(Path::new(&env::var_os("OUT_DIR").unwrap()).join("krom_test.rs"))
        .expect("File creation failed");

    visit_dirs(&base_path, &mut file).unwrap();
}

fn generate_tomharte_65816_test() {
    let mut file =
        File::create(Path::new(&env::var_os("OUT_DIR").unwrap()).join("tomharte_65816.rs"))
            .expect("File creation failed");

    for i in 0..256 {
        let test_name = format!("test_{i:02x}");
        let test_body = format!("run_test(\"{i:02x}.n\");");
        let test = format!("#[test]\nfn {test_name}() {{\n\t{test_body}\n}}\n\n");

        file.write_all(test.as_bytes())
            .expect("Write to file failed");
    }
}

fn generate_krom_test(file: &mut File, path: &str, name: &str) {
    let rom_path = format!("{path}/{name}.sfc");
    let png_path = format!("{path}/{name}.png");
    let test_name = format!("test_{}", name.to_snake_case());
    let test_body =
        format!("compare_to_reference(Path::new(\"{rom_path}\"), Path::new(\"{png_path}\"));",);
    let test = format!("#[test]\nfn {test_name}() {{\n\t{test_body}\n}}\n\n",);

    file.write_all(test.as_bytes())
        .expect("Write to file failed");
}

fn visit_dirs(dir: &Path, file: &mut File) -> std::io::Result<()> {
    if dir.is_dir() {
        let mut has_subdir = false;
        for entry in dir.read_dir()? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                has_subdir = true;
                visit_dirs(&path, file)?;
            }
        }
        if !has_subdir {
            if let (Some(dir_string), Some(file_name)) = (dir.to_str(), dir.file_name()) {
                generate_krom_test(file, dir_string, file_name.to_str().unwrap());
            }
        }
    }
    Ok(())
}
