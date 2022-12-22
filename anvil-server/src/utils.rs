const GIT_SHA: &str = &env!("VERGEN_GIT_SHA");
const GIT_BRANCH: &str = env!("VERGEN_GIT_BRANCH");
const OS: &str = std::env::consts::OS;
const ARCH: &str = std::env::consts::ARCH;

pub const fn build_type() -> &'static str {
    if cfg!(debug_assertions) {
        "dev"
    } else {
        "prod"
    }
}

pub fn version_string() -> String {
    format!(
        "{}@{}/{}",
        GIT_SHA[..7].to_string(),
        GIT_BRANCH,
        &build_type()
    )
}

pub fn os_string() -> String {
    format!("{}-{}", OS, ARCH)
}