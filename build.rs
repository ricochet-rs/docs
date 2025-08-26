use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=src/generated/");

    // Also watch individual generated files
    if Path::new("src/generated/dev").exists() {
        println!("cargo:rerun-if-changed=src/generated/dev/");
    }
    if Path::new("src/generated/v0.1").exists() {
        println!("cargo:rerun-if-changed=src/generated/v0.1/");
    }

    // Watch for any .html files in generated directories
    for entry in std::fs::read_dir("src/generated")
        .unwrap_or_else(|_| {
            std::fs::read_dir(".").unwrap() // fallback to prevent build failure
        })
        .flatten()
    {
        let path = entry.path();
        if path.is_dir() {
            println!("cargo:rerun-if-changed={}", path.display());

            // Watch HTML files in subdirectories
            if let Ok(subdir) = std::fs::read_dir(&path) {
                for sub_entry in subdir.flatten() {
                    if sub_entry
                        .path()
                        .extension()
                        .is_some_and(|ext| ext == "html")
                    {
                        println!("cargo:rerun-if-changed={}", sub_entry.path().display());
                    }
                }
            }
        }
    }
}
