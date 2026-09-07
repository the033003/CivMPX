use crate::civ5::version::sha256_file;
use anyhow::{Context, Result};
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::Path,
};

pub fn inspect_binary(path: &Path) -> Result<()> {
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

    let mut file = File::open(path)
        .with_context(|| format!("unable to open {}", path.display()))?;

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
