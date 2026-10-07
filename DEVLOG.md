# Forge – Devlog

## 2026-10-07 — #1: The foundations

**Why?** I want to learn Rust by building a real, useful tool rather than yet
another throwaway "hello world" CLI. I keep practicing Go in parallel, but this
project belongs to Rust.

**What is Forge?** A cross-platform (macOS, Linux, later Windows) developer and
operations CLI. It works locally, and later it will connect to Linux servers
over SSH, even several at once, without installing anything on the servers.
The goal is to publish it on GitHub and make it installable with `brew install`.

**What I did today**
- Created the Cargo project and set up the module structure:
  `cli` (arguments), `commands` (subcommands), `config` (configuration),
  `utils` (output and styling).
- `clap` (derive) is the only dependency. Coloring is handled by a small
  hand-written ANSI helper instead of pulling in another crate.
- Colors are disabled when output is not a terminal (pipe, file) or when the
  `NO_COLOR` environment variable is set, so Forge stays clean in scripts.
- Commands: `forge --help`, `forge --version`, `forge status`, `forge doctor`.
  `status` and `doctor` only print placeholder output for now.
- The config module already resolves the platform-specific config directory
  (XDG / `~/.config` / `%APPDATA%`), but doesn't read any file yet.

**Decisions**
- Deliberately few dependencies: faster builds, a smaller binary, and I learn
  more by writing the small things myself.
- No SSH, Docker, Git or Claude integration yet. The foundation comes first.
- The `Theme` struct already has its place in the styling module, but there is
  only a single default theme for now.

**Next steps**
- Themes: `github`, `dark`, `light`, `modern`, selectable from config.
- Load a config file (TOML?) — still need to decide if it's worth the dependency.
- Real `doctor` checks (e.g. whether `ssh` and `git` are on the PATH).
- CI (GitHub Actions), release binaries, Homebrew tap.
