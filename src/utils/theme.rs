//! Color themes. Each theme is a small palette of ANSI escape sequences.
//! 256-color codes are used because they work in nearly every terminal.

use clap::ValueEnum;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, ValueEnum)]
pub enum ThemeName {
    #[default]
    Default,
    Github,
    Dark,
    Light,
}

pub struct Theme {
    pub accent: &'static str,
    pub muted: &'static str,
    pub success: &'static str,
    pub warning: &'static str,
    pub error: &'static str,
}

impl ThemeName {
    pub fn palette(self) -> &'static Theme {
        match self {
            Self::Default => &DEFAULT,
            Self::Github => &GITHUB,
            Self::Dark => &DARK,
            Self::Light => &LIGHT,
        }
    }
}

/// Basic ANSI colors, so it follows the terminal's own color scheme.
static DEFAULT: Theme = Theme {
    accent: "\x1b[36m",
    muted: "\x1b[2m",
    success: "\x1b[32m",
    warning: "\x1b[33m",
    error: "\x1b[31m",
};

static GITHUB: Theme = Theme {
    accent: "\x1b[38;5;75m",
    muted: "\x1b[38;5;245m",
    success: "\x1b[38;5;77m",
    warning: "\x1b[38;5;178m",
    error: "\x1b[38;5;203m",
};

static DARK: Theme = Theme {
    accent: "\x1b[38;5;141m",
    muted: "\x1b[38;5;243m",
    success: "\x1b[38;5;114m",
    warning: "\x1b[38;5;221m",
    error: "\x1b[38;5;210m",
};

/// Darker tones that stay readable on a light background.
static LIGHT: Theme = Theme {
    accent: "\x1b[38;5;25m",
    muted: "\x1b[38;5;242m",
    success: "\x1b[38;5;28m",
    warning: "\x1b[38;5;130m",
    error: "\x1b[38;5;160m",
};
