# Roadmap

Where Forge is heading. The order is the plan, not a promise: each milestone
should leave Forge useful on its own before the next one starts.

**Guiding principles**
- One small, fast binary. No runtime, no agent on remote machines.
- Few dependencies. Every new crate has to earn its place.
- Same experience on macOS, Linux and Windows.
- Output that is pleasant for humans and clean for scripts (`NO_COLOR`, pipes).

---

## v0.1 — Foundations ✅

- [x] Cargo project, `clap`-based CLI
- [x] Modular structure: `cli`, `commands`, `config`, `utils`
- [x] `forge status`, `forge doctor` (placeholders)
- [x] Color output with `NO_COLOR` and TTY detection
- [x] Platform-aware config path
- [x] CI on Linux, macOS and Windows

## v0.2 — A real `doctor`

- [ ] Check required tools on `PATH` (`git`, `ssh`) and report their versions
- [ ] Check the config directory and file
- [ ] Clear ✓ / ⚠ / ✗ results with a short hint on how to fix each problem
- [ ] Non-zero exit code when a check fails, so it works in scripts and CI
- [ ] First unit tests

## v0.3 — Config and themes

- [ ] Load `config.toml` (decide: `toml` crate vs. a minimal parser)
- [ ] Themes: `default`, `github`, `dark`, `light`, `modern`
- [ ] `--theme` flag and `theme = "..."` in config
- [ ] `forge config path` / `forge config show`

## v0.4 — Distribution

- [ ] Release workflow: prebuilt binaries for macOS (arm64, x86_64), Linux (x86_64, arm64), Windows (x86_64)
- [ ] GitHub Releases with checksums
- [ ] Homebrew tap: `brew install glitch739/tap/forge`
- [ ] Shell completions (zsh, bash, fish, PowerShell)
- [ ] Install instructions in the README

## v0.5 — Remote hosts over SSH

- [ ] Host inventory in config (name, address, user, port, key)
- [ ] `forge hosts` to list and test connections
- [ ] `forge status --host <name>`: run checks on a Linux server through the system `ssh`, nothing installed remotely
- [ ] `forge doctor --host <name>`

## v0.6 — Many hosts at once

- [ ] Host groups (e.g. `web`, `db`, `prod`)
- [ ] Run a command or check on a group in parallel
- [ ] Summarized, readable output per host
- [ ] Timeouts and per-host error reporting that never hide failures

## Later

- [ ] Git helpers (repo status across projects)
- [ ] Docker helpers (containers, images, cleanup, locally and on remote hosts)
- [ ] AI assistant integration for explaining errors and suggesting fixes
- [ ] Windows as a first-class target, including Windows-specific `doctor` checks

## Not planned

- A daemon or agent running on servers
- A TUI or GUI. Forge stays a plain, scriptable CLI
