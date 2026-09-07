use super::{
    paths,
    version::{SupportedBinary, SupportedVersion, identify_file, sha256_file},
};
use anyhow::{Result, bail};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Civ5Installation {
    pub root: PathBuf,
    pub platform: DetectedPlatform,
    pub version: SupportedVersion,
    pub identity_file: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetectedPlatform {
    Linux,
    Windows,
}

impl DetectedPlatform {
    pub fn name(self) -> &'static str {
        match self {
            Self::Linux => "Linux",
            Self::Windows => "Windows",
        }
    }
}

pub fn detect_installation(root: &Path) -> Result<Civ5Installation> {
    if !root.is_dir() {
        bail!("{} is not a directory", root.display());
    }

    let linux_binary = paths::linux_executable(root);

    if linux_binary.is_file() {
        let version = identify_file(&linux_binary)?;

        if version.binary != SupportedBinary::LinuxCiv5Xp {
            bail!(
                "{} exists but is not the expected Linux Civ V executable",
                linux_binary.display()
            );
        }

        return Ok(Civ5Installation {
            root: root.to_path_buf(),
            platform: DetectedPlatform::Linux,
            version,
            identity_file: linux_binary,
        });
    }

    let windows_binary = paths::windows_database(root);

    if windows_binary.is_file() {
        let version = identify_file(&windows_binary)?;

        if version.binary != SupportedBinary::WindowsCvGameDatabase {
            bail!(
                "{} exists but is not the expected Windows Civ V database DLL",
                windows_binary.display()
            );
        }

        return Ok(Civ5Installation {
            root: root.to_path_buf(),
            platform: DetectedPlatform::Windows,
            version,
            identity_file: windows_binary,
        });
    }

    bail!(
        "could not identify a supported Civilization V installation at {}\n\n\
         Expected one of:\n\
           {}\n\
           {}",
        root.display(),
        paths::linux_executable(root).display(),
        paths::windows_database(root).display()
    )
}

pub fn identify_existing_binary(root: &Path) -> Result<Option<(PathBuf, SupportedVersion)>> {
    let candidates = [
        paths::linux_executable(root),
        paths::linux_original_executable(root),
        paths::windows_database(root),
        paths::windows_original_database(root),
    ];

    for candidate in candidates {
        if !candidate.is_file() {
            continue;
        }

        match sha256_file(&candidate) {
            Ok(hash) => {
                if hash == SupportedVersion::CIV5_1_0_3_279_LINUX.sha256 {
                    return Ok(Some((candidate, SupportedVersion::CIV5_1_0_3_279_LINUX)));
                }

                if hash == SupportedVersion::CIV5_1_0_3_279_WINDOWS.sha256 {
                    return Ok(Some((candidate, SupportedVersion::CIV5_1_0_3_279_WINDOWS)));
                }
            }
            Err(_) => continue,
        }
    }

    Ok(None)
}

pub fn validate_required_linux_files(root: &Path) -> Result<()> {
    let required = [
        paths::linux_executable(root),
        root.join("steamclient.so"),
        root.join("libCvGameCoreDLL.so"),
    ];

    for path in required {
        if !path.is_file() {
            bail!("required Linux Civ V file is missing: {}", path.display());
        }
    }

    Ok(())
}

pub fn validate_required_windows_files(root: &Path) -> Result<()> {
    let required = [
        root.join("CivilizationV.exe"),
        paths::windows_database(root),
        root.join("steam_api.dll"),
    ];

    for path in required {
        if !path.is_file() {
            bail!("required Windows Civ V file is missing: {}", path.display());
        }
    }

    Ok(())
}
