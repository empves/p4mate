use clap::Parser;
use clap::Subcommand;
use std::fs;
use std::io;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "p4mate")]
#[command(about = "A Perforce helper tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Lift the read-only lock on the given directories or files
    Unlock {
        /// Directories or files to unlock
        #[arg(required = true)]
        paths: Vec<String>,
    },
}

fn unlock_path(path: &Path) -> io::Result<()> {
    #[cfg(windows)]
    {
        let meta = fs::metadata(path)?;
        let mut permissions = meta.permissions();
        // Remove the read-only attribute
        permissions.set_readonly(false);
        fs::set_permissions(path, permissions)?;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let meta = fs::metadata(path)?;
        let mut permissions = meta.permissions();
        // Add write bit for owner, group, others
        let mode = permissions.mode();
        let new_mode = mode | 0o222;
        permissions.set_mode(new_mode);
        fs::set_permissions(path, permissions)?;
    }
    Ok(())
}

fn unlock_recursive(path: &Path) {
    if path.is_dir() {
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            let p = entry.path();
            if let Err(e) = unlock_path(p) {
                eprintln!("Failed to unlock {}: {}", p.display(), e);
            }
        }
    } else {
        if let Err(e) = unlock_path(path) {
            eprintln!("Failed to unlock {}: {}", path.display(), e);
        }
    }
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Unlock { paths } => {
            for path in paths {
                let p = Path::new(path);
                unlock_recursive(p);
            }
        }
    }
}
