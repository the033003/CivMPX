use crate::civ5::paths;
use anyhow::{Context, Result};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn run(root: &Path, dry_run: bool) -> Result<()> {
    println!("CivMPX restore");
    println!("==============");
    println!();
    println!("Target:");
    println!("  {}", root.display());
    println!();

    if !root.is_dir() {
        anyhow::bail!("{} is not a directory", root.display());
    }

    let mut actions = Vec::new();

    let linux_original = paths::linux_original_executable(root);
    let linux_current = paths::linux_executable(root);

    if linux_original.is_file() {
        actions.push(("Linux executable", linux_original, linux_current));
    }

    let windows_original = paths::windows_original_database(root);
    let windows_current = paths::windows_database(root);

    if windows_original.is_file() {
        actions.push(("Windows database DLL", windows_original, windows_current));
    }

    if actions.is_empty() {
        println!("No CivMPX backup was found.");
        return Ok(());
    }

    for (name, original, current) in &actions {
        println!("Restore:");
        println!("  {name}");
        println!("  from: {}", original.display());
        println!("  to:   {}", current.display());
        println!();
    }

    if dry_run {
        println!("Dry run requested.");
        println!("No files were modified.");
        return Ok(());
    }

    for (name, original, current) in actions {
        if current.exists() {
            let quarantine = quarantine_path(&current);

            fs::rename(&current, &quarantine).with_context(|| {
                format!(
                    "unable to move current {} to {}",
                    current.display(),
                    quarantine.display()
                )
            })?;

            println!("Moved current {} to {}", name, quarantine.display());
        }

        fs::rename(&original, &current).with_context(|| {
            format!(
                "unable to restore {} from {}",
                current.display(),
                original.display()
            )
        })?;

        println!("Restored {name}");
    }

    println!();
    println!("Restore complete.");

    Ok(())
}

fn quarantine_path(path: &Path) -> PathBuf {
    let mut result = path.to_path_buf();
    result.set_extension("civmpx-removed");
    result
}
