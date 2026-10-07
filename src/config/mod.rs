//! Configuration. Currently only resolves where the config file lives;
//! loading and parsing will come later.

use std::env;
use std::path::PathBuf;

const APP_DIR: &str = "forge";
const FILE_NAME: &str = "config.toml";

#[derive(Debug, Default)]
pub struct Config {
    pub path: Option<PathBuf>,
}

impl Config {
    pub fn load() -> Self {
        Self {
            path: config_dir().map(|dir| dir.join(APP_DIR).join(FILE_NAME)),
        }
    }

    pub fn display_path(&self) -> String {
        self.path
            .as_ref()
            .map_or_else(|| "unavailable".into(), |p| p.display().to_string())
    }
}

/// Platform config directory:
/// `%APPDATA%` on Windows, `$XDG_CONFIG_HOME` or `~/.config` elsewhere.
/// macOS deliberately uses `~/.config`, as most CLI tools do.
fn config_dir() -> Option<PathBuf> {
    if cfg!(windows) {
        return env_path("APPDATA");
    }
    env_path("XDG_CONFIG_HOME").or_else(|| env_path("HOME").map(|home| home.join(".config")))
}

fn env_path(key: &str) -> Option<PathBuf> {
    env::var_os(key)
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
}
