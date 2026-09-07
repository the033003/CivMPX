pub mod detect;
pub mod paths;
pub mod version;

pub use detect::{detect_installation, DetectedPlatform, Civ5Installation};
pub use version::{
    identify_file, sha256_file, SupportedBinary, SupportedVersion,
};
