use std::path::{Path, PathBuf};

pub const STEAM_APP_ID: &str = "8930";

pub fn linux_executable(root: &Path) -> PathBuf {
    root.join("Civ5XP")
}

pub fn linux_original_executable(root: &Path) -> PathBuf {
    root.join("Civ5XP.orig")
}

pub fn windows_executable(root: &Path) -> PathBuf {
    root.join("CivilizationV.exe")
}

pub fn windows_database(root: &Path) -> PathBuf {
    root.join("CvGameDatabaseWin32Final Release.dll")
}

pub fn windows_original_database(root: &Path) -> PathBuf {
    root.join("CvGameDatabaseWin32Final Release.civmpx-original.dll")
}

pub fn legacy_mppatch_backup_dir(root: &Path) -> PathBuf {
    root.join("mppatch-backup")
}

pub fn legacy_mppatch_windows_database(root: &Path) -> PathBuf {
    legacy_mppatch_backup_dir(root).join("CvGameDatabaseWin32Final Release.dll")
}

pub fn legacy_mppatch_original_database(root: &Path) -> PathBuf {
    legacy_mppatch_backup_dir(root).join("CvGameDatabase_Original.dll")
}

pub fn legacy_mppatch_core(root: &Path) -> PathBuf {
    legacy_mppatch_backup_dir(root).join("mppatch_core.dll")
}

pub fn legacy_mppatch_disabled_core(root: &Path) -> PathBuf {
    root.join("mppatch_core.dll.disabled")
}

pub fn legacy_mppatch_marker(root: &Path) -> PathBuf {
    root.join(".mppatch_installer_lock")
}

pub fn legacy_mppatch_config(root: &Path) -> PathBuf {
    legacy_mppatch_backup_dir(root).join("mppatch_config.toml")
}

pub fn legacy_mppatch_log(root: &Path) -> PathBuf {
    root.join("mppatch_debug.log")
}

pub fn assets(root: &Path) -> PathBuf {
    root.join("Assets")
}

pub fn steam_assets(root: &Path) -> PathBuf {
    root.join("steamassets").join("assets")
}

pub fn civ5_config(root: &Path) -> PathBuf {
    root.join("civmpx.toml")
}

pub fn backup_dir(root: &Path) -> PathBuf {
    root.join(".civmpx-backup")
}

pub fn linux_runtime(root: &Path) -> PathBuf {
    root.join("civmpx_runtime.so")
}

pub fn windows_runtime(root: &Path) -> PathBuf {
    root.join("civmpx_runtime.dll")
}

pub fn linux_launcher(root: &Path) -> PathBuf {
    root.join("Civ5XP")
}
