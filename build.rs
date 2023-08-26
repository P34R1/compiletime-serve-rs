use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    // The directory containing the files you want to include
    let input_dir = "dist";

    // The output path for the generated Rust code
    let output_path = Path::new(&std::env::var("OUT_DIR").unwrap()).join("generated_files.rs");
    let mut output_file = fs::File::create(&output_path).unwrap();

    let output_dir = Path::new(&std::env::var("OUT_DIR").unwrap()).join("dist");
    fs::create_dir_all(&output_dir).unwrap();

    // Iterate over files in the input directory
    for entry in fs::read_dir(input_dir).unwrap().flatten() {
        let file_path = entry.path();
        let file_name = file_path.file_name().unwrap().to_string_lossy();

        // Read the contents of the file
        let file_contents = fs::read_to_string(&file_path).unwrap();

        // Generate code that includes the file contents using include_str!
        writeln!(output_file, "#[allow(dead_code)]").unwrap();
        writeln!(
            output_file,
            "pub const {}: &str = include_str!(r\"{}\");",
            sanitize_identifier(&file_name),
            file_path.display()
        )
        .unwrap();

        // Copy the file to the output directory
        let output_file_path = output_dir.join(&*file_name);
        fs::write(&output_file_path, file_contents).unwrap();
    }
}

// Helper function to sanitize file names into valid Rust identifiers
fn sanitize_identifier(name: &str) -> String {
    name.to_uppercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect()
}
