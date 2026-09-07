use std::path::{Path, PathBuf};

pub const STEAM_APP_ID: &str = "8930";

pub fn linux_executable(root: &Path) -> PathBuf {
    root.join("Civ5XP")
}

pub fn linux_original_executable(root: &Path) -> PathBuf {
    root.join("Civ5XP.orig")
}

pub fn windows_database(root: &Path) -> PathBuf {
    root.join("CvGameDatabaseWin32Final Release.dll")
}

pub fn windows_original_database(root: &Path) -> PathBuf {
    root.join("CvGameDatabase_Original.dll")
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
