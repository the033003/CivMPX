use crate::civ5::detect::{
    DetectedPlatform, detect_installation, validate_required_linux_files,
    validate_required_windows_files,
};
use anyhow::{Result, bail};
use std::path::Path;

pub fn run(root: &Path, dry_run: bool) -> Result<()> {
    let installation = detect_installation(root)?;

    println!("CivMPX patch");
    println!("=============");
    println!();
    println!("Target:");
    println!("  {}", installation.root.display());
    println!("Platform:");
    println!("  {}", installation.platform.name());
    println!("Version:");
    println!("  {}", installation.version.version);
    println!("Identity:");
    println!("  {}", installation.identity_file.display());
    println!();

    match installation.platform {
        DetectedPlatform::Linux => {
            validate_required_linux_files(root)?;
        }
        DetectedPlatform::Windows => {
            validate_required_windows_files(root)?;
        }
    }

    println!("Validation:");
    println!("  [OK] supported Civilization V binary");
    println!("  [OK] required files present");
    println!("  [OK] version recognized");
    println!();

    if dry_run {
        println!("Dry run requested.");
        println!();
        println!("No files were modified.");
        return Ok(());
    }

    bail!(
        "CivMPX runtime is not installed yet.\n\n\
         This is intentional: the patcher refuses to modify the game until \
         the matching CivMPX runtime has been built and verified for this \
         exact Civilization V binary.\n\n\
         Nothing was modified."
    )
}
