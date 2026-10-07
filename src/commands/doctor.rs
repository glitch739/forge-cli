//! `forge doctor`: verifies that the local environment can run Forge.

use std::io::ErrorKind;
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
/// A missing or broken tool is reported at `missing_level`.
fn tool(
    label: &'static str,
    name: &str,
    args: &[&str],
    missing_level: Level,
    why: &'static str,
) -> Check {
    let (level, detail) = match Command::new(name).args(args).output() {
        Ok(out) => {
            // Some tools (e.g. `ssh -V`) print their version to stderr.
            let text = if out.stdout.is_empty() {
                &out.stderr
            } else {
                &out.stdout
            };
            let text = String::from_utf8_lossy(text);
            if out.status.success() {
                (Level::Ok, short_version(name, &text))
            } else {
                // Found, but broken, e.g. the macOS `git` stub without Xcode tools.
                let reason = first_line(&text);
                let reason = if reason.is_empty() {
                    format!("exited with {}", out.status)
                } else {
                    reason.to_string()
                };
                (missing_level, format!("not working: {reason}"))
            }
        }
        Err(err) if err.kind() == ErrorKind::NotFound => {
            (missing_level, "not found on PATH".into())
        }
        Err(err) => (missing_level, format!("cannot run: {err}")),
    };

    Check {
        level,
        label,
        detail,
        hint: (level != Level::Ok).then_some(why),
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
    let line = first_line(output);
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

fn first_line(text: &str) -> &str {
    text.lines().next().unwrap_or_default().trim()
}

#[cfg(test)]
mod tests {
    use super::{short_version, tool};
    use crate::utils::Level;

    // These use `cargo`, which is always on PATH while `cargo test` runs.

    #[test]
    fn working_tool_passes() {
        let check = tool("Cargo", "cargo", &["--version"], Level::Fail, "why");
        assert_eq!(check.level, Level::Ok);
        assert!(check.hint.is_none());
    }

    #[test]
    fn failing_tool_is_reported() {
        let check = tool("Cargo", "cargo", &["--no-such-flag"], Level::Fail, "why");
        assert_eq!(check.level, Level::Fail);
        assert!(check.detail.starts_with("not working"), "{}", check.detail);
        assert_eq!(check.hint, Some("why"));
    }

    #[test]
    fn missing_tool_is_not_found() {
        let check = tool("X", "forge-no-such-program", &[], Level::Warn, "why");
        assert_eq!(check.level, Level::Warn);
        assert_eq!(check.detail, "not found on PATH");
    }

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
