# cailoxo

`cailoxo` is a small prompt generator for people who like simple, adaptive prompts across shells.

It reads a TOML config and generates native prompt scripts for shells like `zsh`, PowerShell, and Nushell. The goal is one prompt shape everywhere: OS, path, local Git status, branch, and transient prompt behavior.

## Name

`cailoxo` comes from Vietnamese "cái lò xo", meaning "a spring". The ASCII spelling keeps the project name easy to type in config files, package names, and shell commands.

The name matches the prompt behavior: stretch when the terminal is wide, compress when space is tight, and adapt when the terminal resizes.

## Focus

- TOML config
- native shell output
- adaptive path truncation based on terminal width
- local Git branch and status
- no network fetches during prompt rendering
- simple transient prompt

## Usage

```sh
cailoxo generate --shell zsh
cailoxo generate --shell nu
```

See `docs/configuration.md` for config details.

Quick interactive tests:

```sh
cargo run -- generate --shell zsh; tmp=$(mktemp -d); printf 'source %q/output/prompt.zsh\n' "$PWD" > "$tmp/.zshrc"; ZDOTDIR="$tmp" zsh -i
```

```sh
cargo run -- generate --shell nu; nu --no-config-file --execute 'source output/prompt.nu'
```

## Example

```text
   ~/labs/cailoxo/src   feature/native-prompts !1 
❯
```

## Status

Early project. Design and implementation are still forming.
