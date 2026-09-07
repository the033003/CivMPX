use crate::civ5::paths;
use anyhow::Result;
use std::{
    env,
    path::{Path, PathBuf},
};

pub fn run() -> Result<()> {
    println!("CivMPX locate");
    println!("=============");
    println!();

    let candidates = steam_candidates();

    if candidates.is_empty() {
        println!("No common Steam Civilization V locations were found.");
        return Ok(());
    }

    let mut found = false;

    for candidate in candidates {
        if candidate.is_dir() {
            println!("[FOUND] {}", candidate.display());
            found = true;
        }
    }

    if !found {
        println!("No Civilization V installation was found.");
        println!();
        println!("Checked common Steam locations.");
    }

    Ok(())
}

fn steam_candidates() -> Vec<PathBuf> {
    let mut candidates = Vec::new();

    if let Some(home) = home_dir() {
        candidates.push(
            home.join(".steam")
                .join("steam")
                .join("steamapps")
                .join("common")
                .join("Sid Meier's Civilization V"),
        );

        candidates.push(
            home.join(".local")
                .join("share")
                .join("Steam")
                .join("steamapps")
                .join("common")
                .join("Sid Meier's Civilization V"),
        );

        candidates.push(
            home.join(".steam")
                .join("steam")
                .join("steamapps")
                .join("compatdata")
                .join(paths::STEAM_APP_ID)
                .join("pfx")
                .join("drive_c")
                .join("Program Files")
                .join("Steam")
                .join("steamapps")
                .join("common")
                .join("Sid Meier's Civilization V"),
        );
    }

    if let Ok(steam_root) = env::var("STEAM_ROOT") {
        candidates.push(
            Path::new(&steam_root)
                .join("steamapps")
                .join("common")
                .join("Sid Meier's Civilization V"),
        );
    }

    candidates
}

fn home_dir() -> Option<PathBuf> {
    if let Some(home) = env::var_os("HOME") {
        return Some(PathBuf::from(home));
    }

    dirs::home_dir()
}
