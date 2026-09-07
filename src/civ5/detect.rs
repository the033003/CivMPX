use super::{
    paths,
    version::{SupportedBinary, SupportedVersion, identify_file, sha256_file},
};
use anyhow::{Context, Result, bail};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallationState {
    Clean,
    LegacyMppatch,
}

impl InstallationState {
    pub fn name(self) -> &'static str {
        match self {
            Self::Clean => "Clean",
            Self::LegacyMppatch => "Legacy MPPatch",
        }
    }
}

#[derive(Debug, Clone)]
pub struct InstallationInspection {
    pub installation: Civ5Installation,
    pub state: InstallationState,
}

pub fn detect_installation(root: &Path) -> Result<Civ5Installation> {
    if !root.is_dir() {
        bail!("{} is not a directory", root.display());
    }

    if let Some(installation) = detect_linux_installation(root)? {
        return Ok(installation);
    }

    if let Some(installation) = detect_windows_installation(root)? {
        return Ok(installation);
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

pub fn inspect_installation(root: &Path) -> Result<InstallationInspection> {
    if !root.is_dir() {
        bail!("{} is not a directory", root.display());
    }

    if let Ok(installation) = detect_installation(root) {
        return Ok(InstallationInspection {
            installation,
            state: InstallationState::Clean,
        });
    }

    if let Some(installation) = detect_legacy_mppatch_installation(root)? {
        return Ok(InstallationInspection {
            installation,
            state: InstallationState::LegacyMppatch,
        });
    }

    bail!(
        "could not identify a supported Civilization V installation at {}",
        root.display()
    )
}

fn detect_linux_installation(root: &Path) -> Result<Option<Civ5Installation>> {
    let linux_binary = paths::linux_executable(root);

    if !linux_binary.is_file() {
        return Ok(None);
    }

    let version = identify_file(&linux_binary).with_context(|| {
        format!(
            "unable to identify Linux Civilization V executable {}",
            linux_binary.display()
        )
    })?;

    if version.binary != SupportedBinary::LinuxCiv5Xp {
        bail!(
            "{} exists but is not the expected Linux Civ V executable",
            linux_binary.display()
        );
    }

    Ok(Some(Civ5Installation {
        root: root.to_path_buf(),
        platform: DetectedPlatform::Linux,
        version,
        identity_file: linux_binary,
    }))
}

fn detect_windows_installation(root: &Path) -> Result<Option<Civ5Installation>> {
    let windows_database = paths::windows_database(root);

    if !windows_database.is_file() {
        return Ok(None);
    }

    let version = identify_file(&windows_database).with_context(|| {
        format!(
            "unable to identify Windows Civilization V database {}",
            windows_database.display()
        )
    })?;

    if version.binary != SupportedBinary::WindowsCvGameDatabase {
        bail!(
            "{} exists but is not the expected Windows Civ V database DLL",
            windows_database.display()
        );
    }

    Ok(Some(Civ5Installation {
        root: root.to_path_buf(),
        platform: DetectedPlatform::Windows,
        version,
        identity_file: windows_database,
    }))
}

fn detect_legacy_mppatch_installation(root: &Path) -> Result<Option<Civ5Installation>> {
    let windows_executable = paths::windows_executable(root);
    let backup_database = paths::legacy_mppatch_windows_database(root);

    if !windows_executable.is_file() || !backup_database.is_file() {
        return Ok(None);
    }

    let version = match identify_file(&backup_database) {
        Ok(version) if version.binary == SupportedBinary::WindowsCvGameDatabase => version,
        Ok(_) => return Ok(None),
        Err(_) => return Ok(None),
    };

    let has_mppatch_core = paths::legacy_mppatch_core(root).is_file()
        || paths::legacy_mppatch_disabled_core(root).is_file();

    let has_mppatch_backup = paths::legacy_mppatch_backup_dir(root).is_dir();

    let has_mppatch_marker = paths::legacy_mppatch_marker(root).exists();

    let has_mppatch_artifacts = has_mppatch_core || has_mppatch_backup || has_mppatch_marker;

    if !has_mppatch_artifacts {
        return Ok(None);
    }

    Ok(Some(Civ5Installation {
        root: root.to_path_buf(),
        platform: DetectedPlatform::Windows,
        version,
        identity_file: backup_database,
    }))
}

pub fn identify_existing_binary(root: &Path) -> Result<Option<(PathBuf, SupportedVersion)>> {
    let candidates = [
        paths::linux_executable(root),
        paths::linux_original_executable(root),
        paths::windows_database(root),
        paths::windows_original_database(root),
        paths::legacy_mppatch_windows_database(root),
        paths::legacy_mppatch_original_database(root),
    ];

    for candidate in candidates {
        if !candidate.is_file() {
            continue;
        }

        let hash = match sha256_file(&candidate) {
            Ok(hash) => hash,
            Err(_) => continue,
        };

        if hash == SupportedVersion::CIV5_1_0_3_279_LINUX.sha256 {
            return Ok(Some((candidate, SupportedVersion::CIV5_1_0_3_279_LINUX)));
        }

        if hash == SupportedVersion::CIV5_1_0_3_279_WINDOWS.sha256 {
            return Ok(Some((candidate, SupportedVersion::CIV5_1_0_3_279_WINDOWS)));
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
        paths::windows_executable(root),
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

pub fn validate_legacy_mppatch_files(root: &Path) -> Result<()> {
    let required = [
        paths::windows_executable(root),
        paths::legacy_mppatch_windows_database(root),
    ];

    for path in required {
        if !path.is_file() {
            bail!(
                "required legacy MPPatch file is missing: {}",
                path.display()
            );
        }
    }

    Ok(())
}

pub fn is_legacy_mppatch_installation(root: &Path) -> bool {
    detect_legacy_mppatch_installation(root)
        .ok()
        .flatten()
        .is_some()
}
