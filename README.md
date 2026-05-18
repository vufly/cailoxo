# cailoxo

`cailoxo` is a small prompt generator for people who want one adaptive prompt across shells.

It reads `cailoxo.toml` and generates native prompt scripts for `zsh`, `nu`, and `pwsh`. No prompt framework is required at runtime.

Goal: same prompt shape everywhere: OS, adaptive path, local Git branch/status, and transient prompt behavior.

## Name

`cailoxo` comes from Vietnamese "cái lò xo", meaning "a spring". The ASCII spelling keeps the project name easy to type in config files, package names, and shell commands.

The name matches the prompt behavior: stretch when the terminal is wide, compress when space is tight, and adapt when the terminal resizes.

## Focus

- TOML config
- native shell output for `zsh`, `nu`, and `pwsh`
- adaptive path truncation based on terminal width
- local Git branch and status
- no network fetches during prompt rendering
- simple transient prompt

## Supported Shells

| Shell ID | Shell | Default output |
| --- | --- | --- |
| `zsh` | Zsh | `output/prompt.zsh` |
| `nu` | Nushell | `output/prompt.nu` |
| `pwsh` | PowerShell | `output/prompt.ps1` |

## Usage

Generate prompt scripts:

```sh
cailoxo generate --shell zsh
cailoxo generate --shell nu
cailoxo generate --shell pwsh
```

Source generated scripts from shell startup files:

```zsh
source /path/to/cailoxo/output/prompt.zsh
```

```nu
source /path/to/cailoxo/output/prompt.nu
```

```powershell
. /path/to/cailoxo/output/prompt.ps1
```

Use `--output` to write generated scripts somewhere else.

See `docs/configuration.md` for config details.

Quick interactive tests:

```sh
cargo run -- generate --shell zsh; tmp=$(mktemp -d); printf 'source %q/output/prompt.zsh\n' "$PWD" > "$tmp/.zshrc"; ZDOTDIR="$tmp" zsh -i
```

```sh
cargo run -- generate --shell nu; nu --no-config-file --execute 'source output/prompt.nu'
```

```sh
cargo run -- generate --shell pwsh; pwsh -NoLogo -NoProfile -Command '. ./output/prompt.ps1; prompt'
```

## Example

```text
   ~/labs/cailoxo/src   feature/native-prompts !1 
❯
```

## Release

Releases are tag-driven. Push a version tag to build GitHub release assets and publish to crates.io:

```sh
git tag v0.1.0
git push origin v0.1.0
```

GitHub release assets include `x86_64` and `aarch64` binaries for Linux, macOS, and Windows.

Crates.io publish requires repository secret `CARGO_REGISTRY_TOKEN`.

## Status

Early project. Design and implementation are still forming.
