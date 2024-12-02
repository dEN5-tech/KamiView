use std::fs;
use std::path::Path;
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=resources");

    let out_dir = env::var("OUT_DIR").unwrap();
    let resources_dir = Path::new("resources");
    let target_dir = Path::new(&out_dir).join("resources");

    if !target_dir.exists() {
        fs::create_dir_all(&target_dir).expect("Failed to create resources directory");
    }

    copy_resources(resources_dir, &target_dir);

    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("resources/icon.ico");
        if let Err(e) = res.compile() {
            eprintln!("Failed to compile resources: {}", e);
        }
    }
}

fn copy_resources(from: &Path, to: &Path) {
    if from.is_dir() {
        for entry in fs::read_dir(from).expect("Failed to read directory") {
            let entry = entry.expect("Failed to read entry");
            let path = entry.path();
            
            if path.is_file() {
                let ext = path.extension().and_then(|e| e.to_str());
                if ext.map_or(false, |e| e == "svg" || e == "ttf") {
                    let relative_path = path.strip_prefix(from).unwrap();
                    let target = to.join(relative_path);
                    
                    if let Some(parent) = target.parent() {
                        fs::create_dir_all(parent).expect("Failed to create directory");
                    }
                    
                    fs::copy(&path, &target).expect("Failed to copy file");
                    println!("Copied: {:?} -> {:?}", path, target);
                }
            } else if path.is_dir() {
                let target_dir = to.join(path.file_name().unwrap());
                copy_resources(&path, &target_dir);
            }
        }
    }
} 