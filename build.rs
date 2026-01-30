use std::fs;
use std::path::PathBuf;

fn main() {
    // Get the source directory
    let src_dir = PathBuf::from("src/isle");

    // Find all .isle files in the src directory
    let isle_files: Vec<PathBuf> = fs::read_dir(&src_dir)
        .expect("Failed to read src directory")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()? == "isle" {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    if isle_files.is_empty() {
        println!("cargo:warning=No .isle files found in src/isle directory");
        return;
    }

    // Compile each .isle file
    for isle_file in isle_files {
        let file_name = isle_file.file_name().unwrap().to_str().unwrap();
        let base_name = isle_file.file_stem().unwrap().to_str().unwrap();

        println!("cargo:rerun-if-changed={}", isle_file.display());
        println!("cargo:warning=Compiling ISLE file: {}", file_name);

        // Output the generated Rust code to src/ with the same base name
        let output_file = src_dir.join(format!("{}.rs", base_name));

        // Compile the ISLE file
        match cranelift_isle::compile::from_files(vec![isle_file.clone()], &Default::default()) {
            Ok(code) => {
                if let Err(e) = fs::write(&output_file, code) {
                    eprintln!("Failed to write {}: {}", output_file.display(), e);
                    std::process::exit(1);
                }
                println!("cargo:warning=Generated: {}", output_file.display());
            }
            Err(e) => {
                eprintln!("ISLE compilation failed for {}: {:?}", file_name, e);
                std::process::exit(1);
            }
        }
    }
}
