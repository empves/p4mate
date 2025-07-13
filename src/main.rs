use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::Path;
use walkdir::WalkDir;

fn unlock_path(path: &Path) -> io::Result<()> {
    let mut perms = fs::metadata(path)?.permissions();
    if perms.readonly() {
        perms.set_readonly(false);
        fs::set_permissions(path, perms)?;
        log::info!("Unlocked: {}", path.display());
    }
    Ok(())
}

fn unlock_recursive(path: &Path) {
    if !path.exists() {
        log::error!(
            "Path does not exist, skipping: {}. Please check if the path is correct.",
            path.display()
        );
        return;
    }

    log::info!("Processing path: {}", path.display());

    if path.is_dir() {
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            let p = entry.path();
            if let Err(e) = unlock_path(p) {
                log::error!("Failed to unlock {}: {}. Please check file permissions or if the file is in use.", p.display(), e);
            }
        }
    } else {
        if let Err(e) = unlock_path(path) {
            log::error!("Failed to unlock {}: {}. Please check file permissions or if the file is in use.", path.display(), e);
        }
    }
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let exe_path = match env::current_exe() {
        Ok(path) => path,
        Err(e) => {
            log::error!("Fatal: Could not determine the application's location. Error: {}", e);
            return;
        }
    };

    let exe_dir = match exe_path.parent() {
        Some(dir) => dir,
        None => {
            log::error!("Fatal: Could not determine the application's parent directory.");
            return;
        }
    };

    let paths_to_unlock = [
        "Engine/Build/Build.version",
        "Engine/Programs",
        "Projects/Raid/Saved",
        "Projects/Raid/Plugins/BlockoutToolsPlugin",
        "Projects/Raid/Plugins/ScreenSpaceFogScattering",
    ];

    log::info!("Starting file permission fix-up...");

    for relative_path in &paths_to_unlock {
        let absolute_path = exe_dir.join(relative_path);
        unlock_recursive(&absolute_path);
    }

    log::info!("All paths processed.");
    println!("Press any key to finish...");
    let mut stdin = io::stdin();
    let _ = stdin.read(&mut [0u8]).unwrap();
}
