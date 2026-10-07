# Roadmap

Where Forge is heading. The order is the plan, not a promise: each milestone
should leave Forge useful on its own before the next one starts.

**The idea:** one all-round developer tool that feels the same everywhere.
Forge always runs on my machine. When I work on a Linux server, Forge sends
plain shell commands over `ssh` and renders the results locally, in the same
format as on my own machine. Nothing is installed on the server.

```sh
forge status                  # my machine
forge status --host web1      # the same, on web1
forge use web1                # from now on every command targets web1
forge status --group prod     # several servers at once
```

**Guiding principles**
- One small, fast binary. No runtime, no agent on remote machines.
- Same commands and same look, locally and remotely.
- Few dependencies. Every new crate has to earn its place.
- Works on macOS, Linux and Windows.
- Polished output for humans, clean output for scripts (`NO_COLOR`, pipes).

---

## v0.1 — Foundations ✅

- [x] Cargo project, `clap`-based CLI
- [x] Modular structure: `cli`, `commands`, `config`, `utils`
- [x] `forge status`, `forge doctor` (placeholders)
- [x] Color output with `NO_COLOR` and TTY detection
- [x] Platform-aware config path
- [x] CI on Linux, macOS and Windows

## v0.2 — A real `doctor` ✅

- [x] Check required tools on `PATH` (`git`, `ssh`) and report their versions
- [x] Check the config directory and file
- [x] Clear ✓ / ! / ✗ results with a short hint on how to fix each problem
- [x] Non-zero exit code when a check fails, so it works in scripts and CI
- [x] First unit tests

## v0.3 — Look & feel ✅

- [x] ASCII logo and a redesigned `--help` with examples
- [x] Running `forge` with no command shows the help instead of an error
- [x] `doctor`: system rows, a result summary and timing
- [x] `status`: boxed header card with key–value rows
- [x] Themes: `default`, `github`, `dark`, `light`, selectable with `--theme`
- [x] Global `--no-color` flag

## v0.4 — Targets

The foundation for remote work: every command asks a *target* to run things
instead of spawning processes itself.

- [ ] `Target` trait with a `Local` implementation
- [ ] `doctor` and `status` run everything through the target
- [ ] Load `config.toml` (decide: `toml` crate vs. a minimal parser)
- [ ] Theme selectable from config

## v0.5 — Remote hosts over SSH

- [ ] `Ssh` target using the system `ssh` (no SSH library, no remote install)
- [ ] Host inventory in config (name, address, user, port, key)
- [ ] `--host <name>` on every command
- [ ] `forge use <host>` / `forge use local` to switch the active target
- [ ] `forge hosts` to list hosts and test connections

## v0.6 — Many hosts at once

- [ ] Host groups (e.g. `web`, `db`, `prod`) and `--group <name>`
- [ ] Run checks and commands on a group in parallel
- [ ] Per-host summary, timeouts, and failures that are never hidden

## v0.7 — Project tools

- [ ] Detect the project type (Cargo, Go, npm)
- [ ] `forge build`, `forge test`, `forge lint` using the right toolchain
- [ ] Works the same locally and on a remote target
- [ ] `forge status` shows project and Git information

## v0.8 — Distribution

- [ ] Release workflow: prebuilt binaries for macOS (arm64, x86_64), Linux (x86_64, arm64), Windows (x86_64)
- [ ] GitHub Releases with checksums
- [ ] Homebrew tap: `brew install glitch739/tap/forge`
- [ ] Shell completions (zsh, bash, fish, PowerShell)

## Later

- [ ] Docker helpers (containers, images, cleanup), locally and remotely
- [ ] Git helpers across projects and hosts
- [ ] AI assistant integration for explaining errors and suggesting fixes
- [ ] `--json` output for scripting
- [ ] Windows as a first-class target, including Windows-specific checks

## Not planned

- A daemon or agent running on servers
- A full-screen TUI or a GUI. Forge stays a scriptable CLI with polished output
