use std::{fs::File, io::Write};

fn main() {
    let mut file = File::create("tests/65816.rs").expect("Could not create file");

    file.write_all("mod tomharte;\n\n".as_bytes())
        .expect("Could not write to file");
    file.write_all("use tomharte::run_test;\n\n".as_bytes())
        .expect("Could not write to file");
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
