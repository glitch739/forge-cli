//! Terminal output helpers. All user-facing formatting goes through here,
//! so commands never deal with colors or layout directly.

mod theme;

pub use theme::ThemeName;

use std::env;
use std::io::{self, IsTerminal};
use std::path::Path;
use std::sync::OnceLock;

use theme::Theme;

/// Total width of rules, cards and right-aligned text.
const WIDTH: usize = 52;
/// Width of the label column in fields and checks.
const LABEL: usize = 10;

const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";

struct Output {
    theme: &'static Theme,
    /// Color is allowed at all (no `--no-color`, no `NO_COLOR`).
    allowed: bool,
    /// Color is allowed and stdout is a terminal.
    stdout: bool,
}

static OUTPUT: OnceLock<Output> = OnceLock::new();

/// Sets the theme and color mode. Call once, before printing anything.
pub fn init(theme: ThemeName, no_color: bool) {
    let _ = OUTPUT.set(Output::new(theme, no_color));
}

impl Output {
    fn new(theme: ThemeName, no_color: bool) -> Self {
        let allowed = !no_color && env::var_os("NO_COLOR").is_none();
        Self {
            theme: theme.palette(),
            allowed,
            stdout: allowed && io::stdout().is_terminal(),
        }
    }
}

fn output() -> &'static Output {
    OUTPUT.get_or_init(|| Output::new(ThemeName::default(), false))
}

fn theme() -> &'static Theme {
    output().theme
}

fn paint(style: &str, text: &str) -> String {
    paint_if(output().stdout, style, text)
}

fn paint_if(enabled: bool, style: &str, text: &str) -> String {
    if enabled {
        format!("{style}{text}{RESET}")
    } else {
        text.to_string()
    }
}

/// Display width, counted in characters (good enough for paths and labels).
fn width(text: &str) -> usize {
    text.chars().count()
}

/// Shortens `text` to `max` characters, keeping the end: `…/forge/config.toml`.
fn shorten(text: &str, max: usize) -> String {
    let len = width(text);
    if len <= max {
        return text.to_string();
    }
    let tail: String = text.chars().skip(len + 1 - max).collect();
    format!("…{tail}")
}

/// Outcome of a single check, e.g. in `forge doctor`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Ok,
    Warn,
    Fail,
}

impl Level {
    fn style(self) -> &'static str {
        match self {
            Self::Ok => theme().success,
            Self::Warn => theme().warning,
            Self::Fail => theme().error,
        }
    }

    fn mark(self) -> &'static str {
        match self {
            Self::Ok => "✓",
            Self::Warn => "!",
            Self::Fail => "✗",
        }
    }
}

/// Command title with an optional muted subtitle below it.
pub fn title(title: &str, subtitle: &str) {
    println!("{}", paint(&format!("{BOLD}{}", theme().accent), title));
    if !subtitle.is_empty() {
        println!("{}", paint(theme().muted, subtitle));
    }
    println!();
}

pub fn field(label: &str, value: &str) {
    let label = paint(theme().muted, &format!("{label:<LABEL$}"));
    println!("  {label} {value}");
}

pub fn check(level: Level, label: &str, detail: &str) {
    let mark = paint(level.style(), level.mark());
    println!("  {mark} {label:<LABEL$} {detail}");
}

/// A follow-up line under a check, aligned with its detail column.
pub fn check_hint(text: &str) {
    println!("    {:<LABEL$} {}", "", paint(theme().muted, text));
}

/// A boxed card. The first row is highlighted; right-hand text is accented.
pub fn card(rows: &[(&str, &str)]) {
    let inner = WIDTH - 4;
    let border = |s: &str| paint(theme().muted, s);
    let line = "─".repeat(WIDTH - 2);

    println!("  {}", border(&format!("╭{line}╮")));
    for (i, (left, right)) in rows.iter().enumerate() {
        let left = shorten(left, inner.saturating_sub(width(right) + 1));
        let gap = " ".repeat(inner - width(&left) - width(right));
        let left = if i == 0 {
            paint(BOLD, &left)
        } else {
            paint(theme().muted, &left)
        };
        let right = paint(theme().accent, right);
        println!("  {} {left}{gap}{right} {}", border("│"), border("│"));
    }
    println!("  {}", border(&format!("╰{line}╯")));
}

pub fn rule() {
    println!("  {}", paint(theme().muted, &"─".repeat(WIDTH)));
}

/// A bold section heading with muted text aligned to the right edge.
pub fn section(title: &str, right: &str) {
    let gap = " ".repeat(WIDTH.saturating_sub(width(title) + width(right)));
    println!(
        "  {}{gap}{}",
        paint(BOLD, title),
        paint(theme().muted, right)
    );
}

/// A summary line such as `3 passed`, muted when the count is zero.
pub fn count(level: Level, n: usize, label: &str) {
    let style = if n == 0 { theme().muted } else { level.style() };
    println!("  {}", paint(style, &format!("{n} {label}")));
}

pub fn error(message: &str) {
    let enabled = output().allowed && io::stderr().is_terminal();
    eprintln!("{} {message}", paint_if(enabled, theme().error, "error:"));
}

pub fn platform() -> String {
    format!("{}/{}", env::consts::OS, env::consts::ARCH)
}

/// Shows paths under the home directory as `~/…`.
pub fn tilde(path: &Path) -> String {
    let home = env::var_os("HOME").filter(|h| !h.is_empty());
    match home.and_then(|h| path.strip_prefix(h).ok().map(Path::to_path_buf)) {
        Some(rest) if rest.as_os_str().is_empty() => "~".into(),
        Some(rest) => format!("~/{}", rest.display()),
        None => path.display().to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::shorten;

    #[test]
    fn short_text_is_unchanged() {
        assert_eq!(shorten("config.toml", 20), "config.toml");
    }

    #[test]
    fn long_text_keeps_the_end() {
        assert_eq!(shorten("/very/long/path/config.toml", 12), "…config.toml");
    }
}
