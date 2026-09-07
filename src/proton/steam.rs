use super::prefix;
use std::{fs, path::PathBuf};

pub fn find_civ5_prefixes() -> Vec<PathBuf> {
    let Some(root) = prefix::default_compatdata_root() else {
        return Vec::new();
    };

    if !root.is_dir() {
        return Vec::new();
    }

    let mut result = Vec::new();

    let direct = root.join("8930").join("pfx");

    if direct.is_dir() {
        result.push(direct);
    }

    let Ok(entries) = fs::read_dir(root) else {
        return result;
    };

    for entry in entries.flatten() {
        let candidate = entry.path().join("pfx");

        if candidate.is_dir() {
            result.push(candidate);
        }
    }

    result.sort();
    result.dedup();

    result
}
