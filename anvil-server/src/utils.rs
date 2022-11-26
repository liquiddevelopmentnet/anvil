pub fn get_build_type() -> &'static str {
    if cfg!(debug_assertions) {
        "dev"
    } else {
        "prod"
    }
}

pub fn build_version_string() -> String {
    format!(
        "{}@{}/{}",
        &env!("VERGEN_GIT_SHA")[..7],
        &env!("VERGEN_GIT_BRANCH"),
        &get_build_type()
    )
}

pub fn build_system_info() -> String {
    format!(
        "{}/{}",
        std::env::consts::OS,
        std::env::consts::ARCH
    )
}