//! `forge doctor`: verifies that the local environment can run Forge.

use std::process::Command;
use std::time::Instant;

use super::Result;
use crate::config::Config;
use crate::utils::{self, Level};

struct Check {
    level: Level,
    label: &'static str,
    detail: String,
    hint: Option<&'static str>,
}

pub fn run(config: &Config) -> Result {
    let started = Instant::now();
    utils::title("Forge Doctor", "Checking your development environment...");

    let checks = [
        system(),
        tool(
            "SSH",
            "ssh",
            &["-V"],
            Level::Fail,
            "needed to manage remote hosts",
        ),
        tool(
            "Git",
            "git",
            &["--version"],
            Level::Warn,
            "needed for upcoming Git features",
        ),
        config_file(config),
    ];

    for check in &checks {
        utils::check(check.level, check.label, &check.detail);
        if let Some(hint) = check.hint {
            utils::check_hint(hint);
        }
    }

    let count = |level| checks.iter().filter(|c| c.level == level).count();
    let (passed, warnings, failed) = (count(Level::Ok), count(Level::Warn), count(Level::Fail));

    println!();
    utils::rule();
    let elapsed = format!("{:.2}s", started.elapsed().as_secs_f64());
    utils::section("Result", &elapsed);
    utils::count(Level::Ok, passed, "passed");
    utils::count(Level::Warn, warnings, &plural(warnings, "warning"));
    utils::count(Level::Fail, failed, "failed");

    match failed {
        0 => Ok(()),
        n => Err(format!("{n} {} failed", plural(n, "check"))),
    }
}

fn plural(n: usize, word: &str) -> String {
    if n == 1 {
        word.to_string()
    } else {
        format!("{word}s")
    }
}

fn system() -> Check {
    Check {
        level: Level::Ok,
        label: "System",
        detail: utils::platform(),
        hint: None,
    }
}

/// Looks up an external tool on `PATH` by running it with `args`.
/// A missing tool is reported at `missing_level`.
fn tool(
    label: &'static str,
    name: &str,
    args: &[&str],
    missing_level: Level,
    why: &'static str,
) -> Check {
    match Command::new(name).args(args).output() {
        Ok(out) => {
            // Some tools (e.g. `ssh -V`) print their version to stderr.
            let text = if out.stdout.is_empty() {
                out.stderr
            } else {
                out.stdout
            };
            Check {
                level: Level::Ok,
                label,
                detail: short_version(name, &String::from_utf8_lossy(&text)),
                hint: None,
            }
        }
        Err(_) => Check {
            level: missing_level,
            label,
            detail: "not found on PATH".into(),
            hint: Some(why),
        },
    }
}

fn config_file(config: &Config) -> Check {
    let label = "Config";
    match &config.path {
        Some(path) if path.is_file() => Check {
            level: Level::Ok,
            label,
            detail: utils::tilde(path),
            hint: None,
        },
        Some(path) => Check {
            level: Level::Ok,
            label,
            detail: format!("{} (not created)", utils::tilde(path)),
            hint: None,
        },
        None => Check {
            level: Level::Warn,
            label,
            detail: "no config directory found".into(),
            hint: Some("set HOME (or APPDATA on Windows)"),
        },
    }
}

/// Reduces a tool's version banner to its essentials:
/// `git version 2.39.5 (Apple Git-154)` -> `2.39.5 (Apple Git-154)`,
/// `OpenSSH_9.9p2, LibreSSL 3.3.6` -> `OpenSSH_9.9p2`.
fn short_version(name: &str, output: &str) -> String {
    let line = output.lines().next().unwrap_or_default().trim();
    let line = line
        .strip_prefix(&format!("{name} version "))
        .unwrap_or(line);
    let line = line.split(',').next().unwrap_or(line).trim();
    if line.is_empty() {
        "installed".into()
    } else {
        line.into()
    }
}

#[cfg(test)]
mod tests {
    use super::short_version;

    #[test]
    fn strips_git_prefix() {
        assert_eq!(
            short_version("git", "git version 2.39.5 (Apple Git-154)\n"),
            "2.39.5 (Apple Git-154)"
        );
    }

    #[test]
    fn keeps_first_part_of_ssh_banner() {
        assert_eq!(
            short_version("ssh", "OpenSSH_9.9p2, LibreSSL 3.3.6\n"),
            "OpenSSH_9.9p2"
        );
    }

    #[test]
    fn handles_empty_output() {
        assert_eq!(short_version("ssh", ""), "installed");
    }
}
