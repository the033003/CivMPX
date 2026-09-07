use crate::civ5::{
    detect::{InstallationState, identify_existing_binary, inspect_installation},
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

    match inspect_installation(root) {
        Ok(inspection) => {
            let installation = &inspection.installation;

            println!("Installation:");
            println!("  [OK] Civilization V detected");
            println!("  Platform: {}", installation.platform.name());
            println!("  Version: {}", installation.version.version);
            println!("  State: {}", inspection.state.name());
            println!("  Binary: {}", installation.identity_file.display());
            println!("  SHA-256: {}", installation.version.sha256);

            println!();

            match inspection.state {
                InstallationState::Clean => {
                    println!("Patch state:");
                    println!("  [CLEAN] No CivMPX or legacy MPPatch state detected");
                }

                InstallationState::LegacyMppatch => {
                    println!("Patch state:");
                    println!("  [LEGACY] MPPatch installation detected");

                    println!();
                    println!("Legacy artifacts:");

                    print_artifact(
                        "mppatch-backup/",
                        paths::legacy_mppatch_backup_dir(root).is_dir(),
                    );

                    print_artifact(
                        "mppatch-backup/CvGameDatabaseWin32Final Release.dll",
                        paths::legacy_mppatch_windows_database(root).is_file(),
                    );

                    print_artifact(
                        "mppatch-backup/CvGameDatabase_Original.dll",
                        paths::legacy_mppatch_original_database(root).is_file(),
                    );

                    print_artifact(
                        "mppatch-backup/mppatch_core.dll",
                        paths::legacy_mppatch_core(root).is_file(),
                    );

                    print_artifact(
                        "mppatch_core.dll.disabled",
                        paths::legacy_mppatch_disabled_core(root).is_file(),
                    );

                    print_artifact(
                        ".mppatch_installer_lock",
                        paths::legacy_mppatch_marker(root).exists(),
                    );

                    println!();
                    println!("CivMPX action:");
                    println!(
                        "  [BLOCKED] Restore the legacy MPPatch installation \
                         before using CivMPX patch."
                    );
                }
            }
        }

        Err(_) => {
            println!("Installation:");
            println!("  [--] No supported Civilization V installation detected");
        }
    }

    println!();

    println!("Backups:");

    let linux_backup = paths::linux_original_executable(root);
    let windows_backup = paths::windows_original_database(root);

    if linux_backup.is_file() {
        println!("  [FOUND] {}", linux_backup.display());
    } else {
        println!("  [--] No CivMPX Linux executable backup");
    }

    if windows_backup.is_file() {
        println!("  [FOUND] {}", windows_backup.display());
    } else {
        println!("  [--] No CivMPX Windows database backup");
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

fn print_artifact(name: &str, present: bool) {
    if present {
        println!("  [FOUND] {name}");
    } else {
        println!("  [--] {name}");
    }
}
