mod civ5;
mod cli;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "civmpx",
    version,
    about = "Modern multiplayer extension tooling for Civilization V",
    long_about = "CivMPX inspects, patches, verifies, and restores supported Civilization V installations."
)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Search common Steam locations for Civilization V.
    Locate,

    /// Inspect a Civilization V installation.
    Inspect {
        /// Civilization V installation directory.
        path: PathBuf,
    },

    /// Check the local CivMPX development/runtime environment.
    Doctor,

    /// Show CivMPX patch state for an installation.
    Status {
        /// Civilization V installation directory.
        path: PathBuf,
    },

    /// Apply CivMPX to a supported Civilization V installation.
    Patch {
        /// Civilization V installation directory.
        path: PathBuf,

        /// Do not modify files; only show what would happen.
        #[arg(long)]
        dry_run: bool,
    },

    /// Restore a Civilization V installation previously modified by CivMPX.
    Restore {
        /// Civilization V installation directory.
        path: PathBuf,

        /// Do not modify files; only show what would happen.
        #[arg(long)]
        dry_run: bool,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Locate => cli::locate::run(),
        Command::Inspect { path } => cli::inspect(&path),
        Command::Doctor => cli::doctor(),
        Command::Status { path } => cli::status::run(&path),
        Command::Patch { path, dry_run } => cli::patch::run(&path, dry_run),
        Command::Restore { path, dry_run } => cli::restore::run(&path, dry_run),
    }
}
