use anyhow::{Context, Result, bail};
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportedBinary {
    LinuxCiv5Xp,
    WindowsCvGameDatabase,
}

impl SupportedBinary {
    pub fn description(self) -> &'static str {
        match self {
            Self::LinuxCiv5Xp => "Civ V Linux executable",
            Self::WindowsCvGameDatabase => "Civ V Windows game database DLL",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SupportedVersion {
    pub version: &'static str,
    pub platform: &'static str,
    pub binary: SupportedBinary,
    pub sha256: &'static str,
}

impl SupportedVersion {
    pub const CIV5_1_0_3_279_LINUX: Self = Self {
        version: "1.0.3.279",
        platform: "linux",
        binary: SupportedBinary::LinuxCiv5Xp,
        sha256: "cc06b647821ec5e7cca3c397f6b0d4726f0106cdd67bcf074d494bea2607a8ca",
    };

    pub const CIV5_1_0_3_279_WINDOWS: Self = Self {
        version: "1.0.3.279",
        platform: "windows",
        binary: SupportedBinary::WindowsCvGameDatabase,
        sha256: "f95637398ce10012c785b0dc952686db82613f702a8511bbc7ac822896949563",
    };
}

pub fn sha256_file(path: &Path) -> Result<String> {
    let file = File::open(path).with_context(|| format!("unable to open {}", path.display()))?;

    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 1024 * 1024];

    loop {
        let read = reader
            .read(&mut buffer)
            .with_context(|| format!("unable to read {}", path.display()))?;

        if read == 0 {
            break;
        }

        hasher.update(&buffer[..read]);
    }

    Ok(hex::encode(hasher.finalize()))
}

pub fn identify_file(path: &Path) -> Result<SupportedVersion> {
    if !path.is_file() {
        bail!("{} is not a file", path.display());
    }

    let hash = sha256_file(path)?;

    if hash == SupportedVersion::CIV5_1_0_3_279_LINUX.sha256 {
        return Ok(SupportedVersion::CIV5_1_0_3_279_LINUX);
    }

    if hash == SupportedVersion::CIV5_1_0_3_279_WINDOWS.sha256 {
        return Ok(SupportedVersion::CIV5_1_0_3_279_WINDOWS);
    }

    bail!(
        "unsupported Civilization V binary\n\n\
         File: {}\n\
         SHA-256: {}\n\n\
         CivMPX will not modify an unknown binary.",
        path.display(),
        hash
    )
}
