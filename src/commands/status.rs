use std::env;

use super::Result;
use crate::config::Config;
use crate::utils;

pub fn run(config: &Config) -> Result {
    utils::title("Forge Status", "Local environment");

    let cwd = env::current_dir()
        .map(|dir| utils::tilde(&dir))
        .unwrap_or_default();
    utils::card(&[("local", &utils::platform()), (&cwd, "")]);
    println!();

    let config_path = match &config.path {
        Some(path) if path.is_file() => utils::tilde(path),
        Some(path) => format!("{} (not created)", utils::tilde(path)),
        None => "unavailable".into(),
    };

    utils::field("User", &first_env(&["USER", "USERNAME"]));
    utils::field("Shell", &first_env(&["SHELL", "COMSPEC"]));
    utils::field("Config", &config_path);
    utils::field("Forge", concat!("v", env!("CARGO_PKG_VERSION")));
    Ok(())
}

/// The first non-empty variable among `keys`; covers Unix and Windows names.
fn first_env(keys: &[&str]) -> String {
    keys.iter()
        .find_map(|key| env::var(key).ok().filter(|v| !v.is_empty()))
        .unwrap_or_else(|| "unknown".into())
}
