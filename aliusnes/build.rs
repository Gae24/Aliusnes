use std::{env, fs::File, io::Write, path::Path};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let mut file =
        File::create(Path::new(&env::var_os("OUT_DIR").unwrap()).join("tomharte_65816.rs"))
            .expect("Could not create file");

    for i in 0..256 {
        let attribute = "#[test]".to_string();
        let test_name = format!("test_{i:02x}");
        let test_body = format!("run_test(\"{i:02x}.n\");");
        let test_fn = format!("{attribute}\npub fn {test_name}() {{\n\t{test_body}\n}}\n\n",);

        file.write_all(test_fn.as_bytes())
            .expect("Could not write to file");
    }
}
