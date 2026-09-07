use std::{
    env,
    path::{Path, PathBuf},
};

pub fn default_compatdata_root() -> Option<PathBuf> {
    let home = env::var_os("HOME")?;

    Some(
        PathBuf::from(home)
            .join(".steam")
            .join("steam")
            .join("steamapps")
            .join("compatdata"),
    )
}

pub fn civ5_prefix(root: &Path) -> PathBuf {
    root.join("8930").join("pfx")
}

pub fn windows_drive(prefix: &Path) -> PathBuf {
    prefix.join("drive_c")
}

pub fn exists(prefix: &Path) -> bool {
    prefix.is_dir()
}
