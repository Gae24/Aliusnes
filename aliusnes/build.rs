use std::{env, fs::File, io::Write, path::Path};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let mut file =
        File::create(Path::new(&env::var_os("OUT_DIR").unwrap()).join("tomharte_65816.rs"))
            .expect("Could not create file");

    for i in 0..256 {
        let attribute = "#[test]".to_string();
        let test_name = format!("test_{:02x}", i);
        let test_body = format!("run_test(\"{:02x}.n\");", i);
        let test_fn = format!(
            "{}\npub fn {}() {{\n\t{}\n}}\n\n",
            attribute, test_name, test_body
        );

        file.write_all(test_fn.as_bytes())
            .expect("Could not write to file");
    }
}
