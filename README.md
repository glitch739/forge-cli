# Forge

A cross-platform developer and operations CLI for macOS, Linux and (soon) Windows.

> Early development. Only the foundations are in place. See the [roadmap](ROADMAP.md).

## Build

```sh
cargo build --release
./target/release/forge --help
```

## Install from source

```sh
cargo install --path .
forge
```

## Usage

| Command        | Description                                   |
| -------------- | --------------------------------------------- |
| `forge`        | Show the help with all commands               |
| `forge status` | Show the current state of the environment     |
| `forge doctor` | Check that the environment is set up correctly |

| Option           | Description                                      |
| ---------------- | ------------------------------------------------ |
| `--theme <NAME>` | Color theme: `default`, `github`, `dark`, `light` |
| `--no-color`     | Disable colored output                           |
| `-V, --version`  | Print the version                                |

Colors are also disabled automatically when output is not a terminal or `NO_COLOR` is set.

## License

MIT, see [LICENSE](LICENSE).
