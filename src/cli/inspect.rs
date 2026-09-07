use crate::civ5::{
    detect::{
        inspect_installation, validate_legacy_mppatch_files,
        validate_required_linux_files, validate_required_windows_files,
        DetectedPlatform, InstallationState,
    },
};
use anyhow::Result;
use std::path::Path;

pub fn run(path: &Path) -> Result<()> {
    let inspection = inspect_installation(path)?;
    let installation = &inspection.installation;

    println!("CivMPX Civilization V inspection");
    println!("================================");
    println!();

    println!("Installation:");
    println!("  {}", installation.root.display());
    println!();

    println!("Platform:");
    println!("  {}", installation.platform.name());
    println!();

    println!("Installation state:");
    println!("  {}", inspection.state.name());
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

    match (installation.platform, inspection.state) {
        (DetectedPlatform::Linux, InstallationState::Clean) => {
            println!("Required files:");

            match validate_required_linux_files(path) {
                Ok(()) => {
                    println!("  [OK] Linux installation appears complete");
                }
                Err(error) => {
                    println!("  [FAIL] {error}");
                }
            }
        }

        (DetectedPlatform::Windows, InstallationState::Clean) => {
            println!("Required files:");

            match validate_required_windows_files(path) {
                Ok(()) => {
                    println!("  [OK] Windows installation appears complete");
                }
                Err(error) => {
                    println!("  [FAIL] {error}");
                }
            }

            println!();
            println!("Proton:");
            println!("  A Windows installation may be patched from Linux.");
            println!(
                "  CivMPX treats the game directory independently \
                 from the Proton prefix."
            );
        }

        (DetectedPlatform::Windows, InstallationState::LegacyMppatch) => {
            println!("Legacy MPPatch:");

            match validate_legacy_mppatch_files(path) {
                Ok(()) => {
                    println!(
                        "  [FOUND] Existing legacy MPPatch installation"
                    );
                }
                Err(error) => {
                    println!("  [FAIL] {error}");
                }
            }

            println!();
            println!("Legacy artifacts:");

            print_artifact(
                "mppatch-backup/",
                crate::civ5::paths::legacy_mppatch_backup_dir(path).is_dir(),
            );

            print_artifact(
                "mppatch-backup/CvGameDatabaseWin32Final Release.dll",
                crate::civ5::paths::legacy_mppatch_windows_database(path)
                    .is_file(),
            );

            print_artifact(
                "mppatch-backup/CvGameDatabase_Original.dll",
                crate::civ5::paths::legacy_mppatch_original_database(path)
                    .is_file(),
            );

            print_artifact(
                "mppatch-backup/mppatch_core.dll",
                crate::civ5::paths::legacy_mppatch_core(path).is_file(),
            );

            print_artifact(
                "mppatch_core.dll.disabled",
                crate::civ5::paths::legacy_mppatch_disabled_core(path)
                    .is_file(),
            );

            print_artifact(
                ".mppatch_installer_lock",
                crate::civ5::paths::legacy_mppatch_marker(path).exists(),
            );

            println!();
            println!("CivMPX compatibility:");
            println!("  [INFO] Legacy MPPatch detected.");
            println!(
                "  [SAFE] CivMPX will not modify this installation automatically."
            );
            println!(
                "  [INFO] Remove or restore the legacy patch using its own \
                 restoration process before installing CivMPX."
            );
        }

        (DetectedPlatform::Linux, InstallationState::LegacyMppatch) => {
            unreachable!("legacy MPPatch detection is Windows-only");
        }
    }

    println!();
    println!("Patchability:");

    match inspection.state {
        InstallationState::Clean => {
            println!("  Binary recognized: YES");
            println!("  Runtime payload: NOT YET BUILT");
            println!("  Safe to patch: NO");
        }

        InstallationState::LegacyMppatch => {
            println!("  Binary recognized: YES");
            println!("  Existing legacy patch: YES");
            println!("  CivMPX patch runtime: NOT YET BUILT");
            println!("  Safe to patch: NO");
        }
    }

    println!();
    println!("No game files were modified by this command.");

    Ok(())
}

fn print_artifact(name: &str, present: bool) {
    if present {
        println!("  [FOUND] {name}");
    } else {
        println!("  [--] {name}");
    }
}
