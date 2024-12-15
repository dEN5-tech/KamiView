use std::fs;
use std::path::Path;
use std::env;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/gui/assets");
    println!("cargo:rerun-if-changed=kami-view-front/src");
    println!("cargo:rerun-if-changed=.env");
    
    // Build frontend first
    build_frontend().expect("Failed to build frontend");
    
    // Get the output directory from cargo
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_dir = Path::new(&out_dir);

    // Create required directories in the output
    fs::create_dir_all(dest_dir).unwrap();

    // Copy assets with timestamp check
    copy_assets_with_timestamp(dest_dir);

    // Generate env module
    generate_env_module(dest_dir);
}

fn build_frontend() -> Result<(), Box<dyn std::error::Error>> {
    let frontend_dir = Path::new("kami-view-front");
    
    // Check if pnpm is installed
    let pnpm_check = Command::new("pnpm")
        .arg("--version")
        .output();

    let package_manager = if pnpm_check.is_ok() {
        "pnpm"
    } else {
        // Fallback to npm if pnpm is not available
        "npm"
    };

    println!("Building frontend using {}", package_manager);

    // Install dependencies if node_modules doesn't exist
    if !frontend_dir.join("node_modules").exists() {
        println!("Installing frontend dependencies...");
        let status = Command::new(package_manager)
            .arg("install")
            .current_dir(frontend_dir)
            .status()?;

        if !status.success() {
            return Err("Failed to install frontend dependencies".into());
        }
    }

    // Build frontend
    println!("Building frontend...");
    let status = Command::new(package_manager)
        .arg("run")
        .arg("build")
        .current_dir(frontend_dir)
        .status()?;

    if !status.success() {
        return Err("Frontend build failed".into());
    }

    Ok(())
}

fn copy_assets_with_timestamp(out_dir: &Path) {
    // Define source and destination paths
    let project_root = env::current_dir().expect("Failed to get current directory");
    let frontend_assets = project_root.join("src").join("gui").join("assets");
    
    let assets = [
        (frontend_assets.join("index.html"), "index.html"),
        (frontend_assets.join("icon.svg"), "icon.svg"),
    ];

    for (src_path, dest_name) in assets.iter() {
        let dest_path = out_dir.join(dest_name);

        // Always copy in release mode to ensure fresh files
        #[cfg(not(debug_assertions))]
        {
            println!("Release mode: Copying {} to {}", src_path.display(), dest_path.display());
            copy_file(&src_path, &dest_path)
                .unwrap_or_else(|e| eprintln!("Failed to copy {}: {}", src_path.display(), e));
            continue;
        }

        // In debug mode, use timestamp comparison
        #[cfg(debug_assertions)]
        {
            // Check if source file exists
            if !src_path.exists() {
                eprintln!("Warning: Source file {} does not exist", src_path.display());
                continue;
            }

            // Get source file metadata for timestamp comparison
            let src_metadata = fs::metadata(src_path)
                .expect("Failed to get source file metadata");

            let should_copy = if dest_path.exists() {
                // Compare timestamps if destination exists
                let dest_metadata = fs::metadata(&dest_path)
                    .expect("Failed to get destination file metadata");
                
                src_metadata.modified().unwrap() > dest_metadata.modified().unwrap()
            } else {
                true
            };

            if should_copy {
                println!("Copying {} to {}", src_path.display(), dest_path.display());
                copy_file(&src_path, &dest_path)
                    .unwrap_or_else(|e| eprintln!("Failed to copy {}: {}", src_path.display(), e));
            }
        }
    }
}

fn copy_file(src: &Path, dest: &Path) -> std::io::Result<()> {
    println!("cargo:rerun-if-changed={}", src.display());
    
    // Create parent directories if they don't exist
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Copy the file
    fs::copy(src, dest)?;

    Ok(())
}

fn generate_env_module(out_dir: &Path) {
    // Read .env file
    let env_content = fs::read_to_string(".env").unwrap_or_default();
    let mut env_vars = Vec::new();

    // Parse .env file
    for line in env_content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim().to_lowercase()
                .replace("_", ""); // Remove underscores for field names
            env_vars.push((key, value.trim()));
        }
    }

    // Generate Rust module
    let mut module_content = String::from(
        "/// Auto-generated environment variables module\n\
         #[allow(dead_code)]\n\
         pub struct EnvVars {\n"
    );

    // Add fields
    for (key, _) in &env_vars {
        module_content.push_str(&format!("    pub {}: String,\n", key));
    }

    module_content.push_str("}\n\n");

    // Add implementation
    module_content.push_str(
        "impl EnvVars {\n\
         #[allow(dead_code)]\n\
         pub fn new() -> Self {\n\
         Self {\n"
    );

    // Add field initializations
    for (key, value) in &env_vars {
        module_content.push_str(&format!(
            "            {}: \"{}\".to_string(),\n",
            key,
            value.replace("\"", "\\\"")
        ));
    }

    module_content.push_str(
        "        }\n\
         }\n\
         }\n"
    );

    // Write the module to a file
    let out_file = out_dir.join("env.rs");
    fs::write(&out_file, module_content).expect("Failed to write env module");
}