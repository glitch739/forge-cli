//! Terminal output helpers. All user-facing formatting goes through here,
//! so themes can be added later without touching the commands.

use std::env;
use std::io::{self, IsTerminal};
use std::sync::OnceLock;

/// Color palette. A single default for now; more themes will follow.
struct Theme {
    accent: &'static str,
    muted: &'static str,
    success: &'static str,
    error: &'static str,
}

const DEFAULT_THEME: Theme = Theme {
    accent: "\x1b[1;36m",
    muted: "\x1b[2m",
    success: "\x1b[32m",
    error: "\x1b[1;31m",
};

const RESET: &str = "\x1b[0m";

fn color_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| env::var_os("NO_COLOR").is_none() && io::stdout().is_terminal())
}

fn paint(style: &str, text: &str) -> String {
    if color_enabled() {
        format!("{style}{text}{RESET}")
    } else {
        text.to_string()
    }
}

fn theme() -> &'static Theme {
    &DEFAULT_THEME
}

pub fn header(title: &str) {
    println!("{} {}", paint(theme().accent, "forge"), paint(theme().muted, title));
    println!();
}

pub fn field(label: &str, value: &str) {
    println!("  {} {value}", paint(theme().muted, &format!("{label:<10}")));
}

pub fn check(label: &str, ok: bool) {
    let mark = if ok {
        paint(theme().success, "✓")
    } else {
        paint(theme().error, "✗")
    };
    println!("  {mark} {label}");
}

pub fn hint(text: &str) {
    println!("  {}", paint(theme().muted, text));
}

pub fn error(message: &str) {
    eprintln!("{} {message}", paint(theme().error, "error:"));
}

pub fn platform() -> String {
    format!("{}/{}", env::consts::OS, env::consts::ARCH)
}
