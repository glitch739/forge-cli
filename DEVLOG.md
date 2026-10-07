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

## 2026-10-07 — #2: First build and CI

**What I did**
- Installed Rust via rustup and ran the first build. Everything compiled on
  the first try, and `clippy -D warnings` came back clean.
- Added the MIT license and committed `Cargo.lock`. For a binary crate the
  lockfile belongs in the repo, so CI and releases build the exact same
  dependency versions as my machine.
- Applied `rustfmt`.
- Set up GitHub Actions CI: format check, plus clippy, tests and a smoke run
  of `status` and `doctor` on Linux, macOS and Windows.

**Why CI this early?** I don't have a Windows machine. CI is the cheapest way
to find out whether Forge actually builds and runs there, on every push,
instead of discovering it at release time.

**Learned**
- `cargo clippy -- -D warnings` and `cargo fmt --check` are my pre-commit
  routine from now on.
- `cargo run -- <args>`: everything after `--` goes to my program, not to cargo.

## 2026-10-07 — #3: A real `doctor`

**What I did**
- Wrote a roadmap (`ROADMAP.md`) so I know what each version is about.
- `forge doctor` now does real checks:
  - `ssh`: required for remote hosts later, so a missing one is a failure.
  - `git`: only needed for future features, so missing it is just a warning.
  - config: shows where the config file is expected and whether it exists.
- Each problem gets a one-line hint, and there's a summary at the end.
- `doctor` exits with code 1 if any check fails, so it can be used in scripts.
- Split `commands` into one module per subcommand (`status.rs`, `doctor.rs`).
- Error messages on stderr now decide on color based on stderr, not stdout.
- First unit tests, for the version-string parsing.

**Learned**
- `std::process::Command` finds programs on `PATH` on every platform
  (`ssh.exe` on Windows too), and a missing program comes back as an `Err`
  from `.output()`. No extra crate needed.
- `ssh -V` prints its version to stderr, not stdout. Fun.
- `#[cfg(test)] mod tests` keeps tests next to the code without shipping them
  in the release binary.
- Quick way to test the failure path: `PATH=/nonexistent ~/.cargo/bin/cargo run -- doctor`.

## 2026-10-07 — #4: Look & feel

**Rethinking the roadmap.** I made the vision sharper: Forge always runs on
my machine. On a Linux server it sends plain shell commands over `ssh` and
renders the results locally, in exactly the same format. `forge use web1`
will switch the target, like a `kubectl` context. Before any SSH work, v0.4
introduces a `Target` layer, so remote support becomes "just another target"
instead of a rewrite. Project tools (`build`, `test`, `lint` for Cargo, Go and
npm) moved into the plan as v0.7.

**CLI vs. TUI vs. GUI.** A CLI can look great: colors, Unicode boxes, an ASCII
logo. A TUI is a full-screen interactive app (`htop`, `lazygit`), a GUI is a
windowed app. Forge stays a CLI, but a good-looking one.

**What I did (v0.3)**
- ASCII `FORGE` logo with the version in `--help`; running plain `forge` now
  shows the help instead of an error.
- Help with colored sections and an Examples block (clap `Styles`,
  `before_help`, `after_help`).
- `doctor`: title, a System row, a separator and a Result block with colored
  counts and the elapsed time.
- `status`: a boxed card (`╭─╮`) with the target and working directory,
  followed by user, shell, config path and version.
- Themes: `default`, `github`, `dark`, `light`, chosen with `--theme`.
  256-color palettes, so they work in almost every terminal.
- Global `--no-color` flag; paths under the home directory are shown as `~/…`.

**Learned**
- Help is rendered by clap *during* parsing, before my flags exist. That's why
  `--no-color` is checked in the raw arguments up front.
- `OnceLock` is a clean way to set the theme once and read it from anywhere,
  without passing it to every function.
- Box drawing needs widths computed on the plain text. Escape codes have
  length but no width, so color is applied after the padding.
- Nerd Font icons look great in screenshots, but show up as empty boxes for
  anyone without the font. I stuck to plain Unicode.


## 2026-10-07 — #5: Next steps

**Repo housekeeping.** An early commit had an unwanted bug, and GitHub kept showing it even after I rewrote the commit. 
The cleanest fix was to delete and recreate the repo from my clean local history.

**Plan for tomorrow evening: v0.4 Targets**
The goal is that commands no longer spawn processes themselves. They ask a
*target* to run things, so SSH support later is just another target.

- [ ] Define a `Target` trait: run a command, get stdout/stderr/exit code back
- [ ] Implement `Local` using `std::process::Command`
- [ ] Move `doctor` and `status` onto the target (no direct `Command` calls left)
- [ ] Show the active target in `status` (`local` for now)
- [ ] A few unit tests with a fake target, to test `doctor` without real tools
- [ ] Bump to v0.4.0, update ROADMAP and DEVLOG
- [ ] Decide on config loading: `toml` crate vs. a tiny hand-written parser

**Things I want to learn along the way**
- Traits and trait objects (`&dyn Target` vs. generics)
- Writing my own error type instead of passing `String`s around
- How to test code that talks to the outside world (fakes/mocks)
