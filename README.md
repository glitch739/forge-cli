# Forge

A cross-platform developer and operations CLI for macOS, Linux and (soon) Windows.

> Early development. Only the foundations are in place.

## Build

```sh
cargo build --release
./target/release/forge --help
```

## Commands

| Command          | Description                                   |
| ---------------- | --------------------------------------------- |
| `forge status`   | Show the current state of the environment     |
| `forge doctor`   | Check that the local environment is set up    |
| `forge --version`| Print the version                             |

Colors are disabled automatically when output is not a terminal or `NO_COLOR` is set.
