use crate::civ5::detect::{
DetectedPlatform, InstallationState, inspect_installation, validate_required_linux_files,
validate_required_windows_files,
};
use crate::civ5::paths;
use crate::civ5::version::{SupportedBinary, sha256_file};
use anyhow::{Context, Result, bail};
use std::{
fs,
io::Write,
path::{Path, PathBuf},
time::{SystemTime, UNIX_EPOCH},
};

const BACKUP_DIR: &str = ".civmpx-backup";
const MARKER_FILE: &str = ".civmpx-installed";
const WINDOWS_DATABASE: &str = "CvGameDatabaseWin32Final Release.dll";

pub fn run(root: &Path, dry_run: bool) -> Result<()> {
let inspection = inspect_installation(root)?;
let installation = &inspection.installation;

println!("CivMPX patch");
println!("============");
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

if inspection.state == InstallationState::LegacyMppatch {
    bail!(
        "legacy MPPatch installation detected.\n\n\
         Restore Civilization V before using CivMPX."
    );
}

match installation.platform {
    DetectedPlatform::Linux => validate_required_linux_files(root)?,
    DetectedPlatform::Windows => validate_required_windows_files(root)?,
}

if marker_path(root).is_file() {
    bail!(
        "CivMPX is already installed at {}.\n\
         Use `civmpx restore {}` first.",
        root.display(),
        root.display()
    );
}

let target = installation.identity_file.clone();
let original_hash = sha256_file(&target)?;

println!("Validation:");
println!("  [OK] supported Civilization V binary");
println!("  [OK] required files present");
println!("  [OK] version recognized");
println!("  [OK] binary hash {}", original_hash);
println!();

if dry_run {
    println!("Dry run requested.");
    println!();
    println!("Would create backup:");
    println!("  {}", backup_path(root, &target).display());
    println!();
    println!("Would install CivMPX marker:");
    println!("  {}", marker_path(root).display());
    println!();
    println!("No files were modified.");
    return Ok(());
}

create_backup(root, &target)?;
write_marker(root, installation.platform.name(), &target, &original_hash)?;

/*
 * The repository currently contains no CivMPX runtime or byte-level
 * patch definitions. Do not corrupt a valid Civilization V binary.
 *
 * The backup and marker are deliberately removed if no actual runtime
 * was installed.
 */
remove_marker(root)?;
remove_backup(root, &target)?;

bail!(
    "CivMPX runtime is not present in this build.\n\n\
     The game binary was NOT modified and the temporary backup was removed.\n\
     No changes were left in the Civilization V installation.\n\n\
     A real runtime/patch definition is required before CivMPX can enable \
     modded multiplayer."
)


}

fn backup_path(root: &Path, target: &Path) -> PathBuf {
let file_name = target
.file_name()
.unwrap_or_default()
.to_string_lossy()
.into_owned();

root.join(BACKUP_DIR).join(file_name)


}

fn marker_path(root: &Path) -> PathBuf {
root.join(MARKER_FILE)
}

fn create_backup(root: &Path, target: &Path) -> Result<()> {
let backup = backup_path(root, target);

if backup.exists() {
    bail!(
        "backup already exists at {}. Refusing to overwrite it.",
        backup.display()
    );
}

let parent = backup
    .parent()
    .context("backup path has no parent directory")?;

fs::create_dir_all(parent)
    .with_context(|| format!("unable to create {}", parent.display()))?;

fs::copy(target, &backup).with_context(|| {
    format!(
        "unable to back up {} to {}",
        target.display(),
        backup.display()
    )
})?;

let original_hash = sha256_file(target)?;
let backup_hash = sha256_file(&backup)?;

if original_hash != backup_hash {
    let _ = fs::remove_file(&backup);
    bail!(
        "backup verification failed.\n\
         Original SHA-256: {original_hash}\n\
         Backup SHA-256:   {backup_hash}"
    );
}

Ok(())


}

fn remove_backup(root: &Path, target: &Path) -> Result<()> {
let backup = backup_path(root, target);

if backup.is_file() {
    fs::remove_file(&backup)
        .with_context(|| format!("unable to remove {}", backup.display()))?;
}

let dir = root.join(BACKUP_DIR);

if dir.is_dir() {
    let mut entries = fs::read_dir(&dir)
        .with_context(|| format!("unable to read {}", dir.display()))?;

    if entries.next().transpose()?.is_none() {
        fs::remove_dir(&dir)
            .with_context(|| format!("unable to remove {}", dir.display()))?;
    }
}

Ok(())


}

fn write_marker(
root: &Path,
platform: &str,
target: &Path,
original_hash: &str,
) -> Result<()> {
let marker = marker_path(root);

let timestamp = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap_or_default()
    .as_secs();

let mut file = fs::File::create(&marker)
    .with_context(|| format!("unable to create {}", marker.display()))?;

writeln!(file, "CivMPX")?;
writeln!(file, "platform={platform}")?;
writeln!(file, "target={}", target.display())?;
writeln!(file, "original_sha256={original_hash}")?;
writeln!(file, "created_unix={timestamp}")?;

file.sync_all()
    .with_context(|| format!("unable to flush {}", marker.display()))?;

Ok(())


}

fn remove_marker(root: &Path) -> Result<()> {
let marker = marker_path(root);

if marker.is_file() {
    fs::remove_file(&marker)
        .with_context(|| format!("unable to remove {}", marker.display()))?;
}

Ok(())


}

#[allow(dead_code)]
fn _supported_target(target: &Path) -> Result<()> {
let version = crate::civ5::version::identify_file(target)?;

match version.binary {
    SupportedBinary::LinuxCiv5Xp | SupportedBinary::WindowsCvGameDatabase => Ok(()),
}


}

#[allow(dead_code)]
fn _windows_database(root: &Path) -> PathBuf {
paths::windows_database(root)
}

#[allow(dead_code)]
fn _database_name() -> &'static str {
WINDOWS_DATABASE
}