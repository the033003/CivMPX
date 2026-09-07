pub mod locate;
pub mod patch;
pub mod restore;
pub mod status;

use crate::civ5::{detect::detect_installation, version::sha256_file};
use anyhow::{Context, Result};
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::Path,
};

pub fn inspect(path: &Path) -> Result<()> {
    if !path.is_file() {
        return inspect_installation(path);
    }

    inspect_binary(path)
}

fn inspect_installation(path: &Path) -> Result<()> {
    let installation = detect_installation(path)?;

    println!("Civilization V installation");
    println!("===========================");
    println!();
    println!("Path:");
    println!("  {}", installation.root.display());
    println!();
    println!("Platform:");
    println!("  {}", installation.platform.name());
    println!();
    println!("Version:");
    println!("  {}", installation.version.version);
    println!();
    println!("Binary:");
    println!("  {}", installation.version.binary.description());
    println!();
    println!("Identity file:");
    println!("  {}", installation.identity_file.display());
    println!();
    println!("SHA-256:");
    println!("  {}", installation.version.sha256);

    Ok(())
}

fn inspect_binary(path: &Path) -> Result<()> {
    let hash = sha256_file(path)?;

    println!("Binary inspection");
    println!("=================");
    println!();
    println!("File:");
    println!("  {}", path.display());
    println!();
    println!("Size:");
    println!("  {} bytes", std::fs::metadata(path)?.len());
    println!();
    println!("SHA-256:");
    println!("  {hash}");
    println!();

    let mut file =
        File::open(path).with_context(|| format!("unable to open {}", path.display()))?;

    let mut magic = [0u8; 64];
    let read = file.read(&mut magic)?;

    println!("Magic:");
    print!("  ");

    for byte in &magic[..read.min(16)] {
        print!("{byte:02x} ");
    }

    println!();

    file.seek(SeekFrom::Start(0))?;

    if read >= 2 && &magic[..2] == b"MZ" {
        println!("Format:");
        println!("  PE / Windows executable");
    } else if read >= 4 && &magic[..4] == b"\x7fELF" {
        println!("Format:");
        println!("  ELF / Linux executable");
    } else {
        println!("Format:");
        println!("  Unknown");
    }

    Ok(())
}

pub fn doctor() -> Result<()> {
    println!("CivMPX doctor");
    println!("=============");
    println!();

    println!("Build:");
    println!("  [OK] CivMPX executable is running");
    println!();

    println!("Supported binaries:");

    for version in [
        crate::civ5::version::SupportedVersion::CIV5_1_0_3_279_LINUX,
        crate::civ5::version::SupportedVersion::CIV5_1_0_3_279_WINDOWS,
    ] {
        println!(
            "  [OK] {} {} ({})",
            version.platform,
            version.version,
            version.binary.description()
        );
    }

    println!();
    println!("Runtime:");
    println!("  [INFO] CivMPX patch runtime is not implemented yet.");
    println!();
    println!("Status:");
    println!("  Research and reverse-engineering phase.");

    Ok(())
}
