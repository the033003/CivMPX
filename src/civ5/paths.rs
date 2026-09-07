use std::path::{Path, PathBuf};

pub const STEAM_APP_ID: &str = "8930";

pub fn linux_executable(root: &Path) -> PathBuf {
    root.join("Civ5XP")
}

pub fn linux_original_executable(root: &Path) -> PathBuf {
    root.join("Civ5XP.civmpx-original")
}

pub fn windows_database(root: &Path) -> PathBuf {
    root.join("CvGameDatabaseWin32Final Release.dll")
}

pub fn windows_original_database(root: &Path) -> PathBuf {
    root.join("CvGameDatabaseWin32Final Release.civmpx-original.dll")
}
