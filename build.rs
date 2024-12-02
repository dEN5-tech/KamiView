use std::fs;
use std::path::Path;
use std::env;

#[cfg(all(target_os = "windows", feature = "windows_build"))]
fn main() {
    use std::io::Write;
    use winres::WindowsResource;

    let mut res = WindowsResource::new();
    res.set_icon("resources/icon.ico")
        .set("ProductName", "KamiView")
        .set("FileDescription", "A modern anime streaming app")
        .set("LegalCopyright", "© 2024")
        .set_manifest(r#"
        <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
            <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
                <security>
                    <requestedPrivileges>
                        <requestedExecutionLevel level="asInvoker" uiAccess="false"/>
                    </requestedPrivileges>
                </security>
            </trustInfo>
            <compatibility xmlns="urn:schemas-microsoft-com:compatibility.v1">
                <application>
                    <supportedOS Id="{8e0f7a12-bfb3-4fe8-b9a5-48fd50a15a9a}"/>
                </application>
            </compatibility>
        </assembly>
        "#);

    if let Err(e) = res.compile() {
        writeln!(std::io::stderr(), "Error: {}", e).unwrap();
        std::process::exit(1);
    }
}

#[cfg(not(all(target_os = "windows", feature = "windows_build")))]
fn main() {
    // Linux build steps
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=PKG_CONFIG_PATH");
    
    // Link required libraries
    println!("cargo:rustc-link-lib=mpv");
    println!("cargo:rustc-link-lib=X11");
    println!("cargo:rustc-link-lib=gtk-3");
    println!("cargo:rustc-link-lib=gdk-3");
    println!("cargo:rustc-link-lib=cairo");
    println!("cargo:rustc-link-lib=pango-1.0");
    
    // Set pkg-config path
    if std::env::var("PKG_CONFIG_PATH").is_err() {
        std::env::set_var(
            "PKG_CONFIG_PATH",
            "/usr/lib/pkgconfig:/usr/share/pkgconfig:/usr/lib/x86_64-linux-gnu/pkgconfig"
        );
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