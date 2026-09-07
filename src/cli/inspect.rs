use crate::civ5::{
    detect::{detect_installation, validate_required_linux_files, validate_required_windows_files},
    DetectedPlatform,
};
use anyhow::Result;
use std::path::Path;

pub fn run(path: &Path) -> Result<()> {
    let installation = detect_installation(path)?;

    println!("CivMPX Civilization V inspection");
    println!("================================");
    println!();
    println!("Installation:");
    println!("  {}", installation.root.display());
    println!();
    println!("Platform:");
    println!("  {}", installation.platform.name());
    println!();
    println!("Game version:");
    println!("  {}", installation.version.version);
    println!();
    println!("Identity binary:");
    println!("  {}", installation.identity_file.display());
    println!();
    println!("SHA-256:");
    println!("  {}", installation.version.sha256);
    println!();

    match installation.platform {
        DetectedPlatform::Linux => {
            println!("Required files:");
            match validate_required_linux_files(path) {
                Ok(()) => println!("  [OK] Linux installation appears complete"),
                Err(error) => println!("  [FAIL] {error}"),
            }
        }

        DetectedPlatform::Windows => {
            println!("Required files:");
            match validate_required_windows_files(path) {
                Ok(()) => println!("  [OK] Windows installation appears complete"),
                Err(error) => println!("  [FAIL] {error}"),
            }

            println!();
            println!("Proton:");
            println!("  A Windows installation may be patched from Linux.");
            println!("  CivMPX will treat the game directory independently from the Proton prefix.");
        }

        DetectedPlatform::Unknown => {
            println!("  [FAIL] Unknown platform");
        }
    }

    println!();
    println!("Patchability:");
    println!("  Binary recognized: YES");
    println!("  Runtime payload: NOT YET BUILT");
    println!("  Safe to patch: NO");
    println!();
    println!(
        "No game files were modified by this command."
    );

    Ok(())
}
