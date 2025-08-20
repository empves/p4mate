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

fn unlock_subdir_recursive(start_path: &Path, target_dir_name: &str) {
    if !start_path.exists() {
        log::error!(
            "Start path does not exist: {}. Please check if the path is correct.",
            start_path.display()
        );
        return;
    }

    log::info!("Searching for directories named '{}' starting from: {}", target_dir_name, start_path.display());

    let mut found_count = 0;

    for entry in WalkDir::new(start_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        
        // Check if this is a directory and if its name matches our target
        if path.is_dir() {
            if let Some(dir_name) = path.file_name() {
                if dir_name == target_dir_name {
                    log::info!("Found target directory: {}", path.display());
                    unlock_recursive(path);
                    found_count += 1;
                }
            }
        }
    }

    if found_count == 0 {
        log::warn!("No directories named '{}' found in: {}", target_dir_name, start_path.display());
    } else {
        log::info!("Successfully processed {} directories named '{}'", found_count, target_dir_name);
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

    let intermediate_parent_dirs = [
        "Projects/Raid/Plugins"
    ];

    log::info!("Starting fixing Intermediate directories...");

    for parent_dir in &intermediate_parent_dirs {
        let absolute_path = exe_dir.join(parent_dir);
        unlock_subdir_recursive(&absolute_path, "Intermediate");
    }

    log::info!("All paths processed.");
    println!("Press any key to finish...");
    let mut stdin = io::stdin();
    let _ = stdin.read(&mut [0u8]).unwrap();
}
