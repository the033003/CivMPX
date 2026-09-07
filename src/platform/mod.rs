pub fn current_platform() -> &'static str {
    std::env::consts::OS
}

pub fn current_architecture() -> &'static str {
    std::env::consts::ARCH
}
