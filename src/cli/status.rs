use crate::civ5::{
    detect::{detect_installation, identify_existing_binary},
    paths,
};
use anyhow::Result;
use std::path::Path;

pub fn run(root: &Path) -> Result<()> {
    println!("CivMPX status");
    println!("=============");
    println!();
    println!("Target:");
    println!("  {}", root.display());
    println!();

    if let Ok(installation) = detect_installation(root) {
        println!("Installation:");
        println!("  [OK] Civilization V detected");
        println!("  Platform: {}", installation.platform.name());
        println!("  Version: {}", installation.version.version);
        println!("  Binary: {}", installation.identity_file.display());
        println!("  SHA-256: {}", installation.version.sha256);
    } else {
        println!("Installation:");
        println!("  [--] No supported unmodified Civ V binary detected");
    }

    println!();

    let linux_backup = paths::linux_original_executable(root);
    let windows_backup = paths::windows_original_database(root);

    println!("Backups:");

    if linux_backup.is_file() {
        println!("  [FOUND] {}", linux_backup.display());
    } else {
        println!("  [--] No Linux executable backup");
    }

    if windows_backup.is_file() {
        println!("  [FOUND] {}", windows_backup.display());
    } else {
        println!("  [--] No Windows database backup");
    }

    println!();

    match identify_existing_binary(root)? {
        Some((path, version)) => {
            println!("Recognized binary:");
            println!("  {}", path.display());
            println!("  {} {}", version.platform, version.version);
        }
        None => {
            println!("Recognized binary:");
            println!("  None");
        }
    }

    println!();
    println!("Runtime:");
    println!("  CivMPX patch runtime is not implemented yet.");

    Ok(())
}
