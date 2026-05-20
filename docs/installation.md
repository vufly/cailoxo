# Installation

`cailoxo` is distributed as a Rust crate and as GitHub release binaries.

## Cargo

```sh
cargo install cailoxo
```

Generate shell scripts:

```sh
cailoxo generate --shell zsh
cailoxo generate --shell nu
cailoxo generate --shell pwsh
```

Default outputs:

- `output/prompt.zsh`
- `output/prompt.nu`
- `output/prompt.ps1`

Use `--output` for a stable location, such as `~/.config/cailoxo/prompt.zsh`.

Use `--config` when your TOML file lives somewhere else:

```sh
cailoxo generate --shell zsh --config ~/.config/cailoxo/theme.toml --output ~/.config/cailoxo/prompt.zsh
```

## GitHub Release Binaries

Download a binary from the latest GitHub release, then put it on `PATH`.

Release assets are built for:

- Linux `x86_64` and `aarch64`
- macOS `x86_64` and `aarch64`
- Windows `x86_64` and `aarch64`

## Shell Setup

Generate once after editing `cailoxo.toml`, then source the generated script from shell startup.

zsh:

```zsh
source ~/.config/cailoxo/prompt.zsh
```

Nushell:

```nu
source ~/.config/cailoxo/prompt.nu
```

PowerShell:

```powershell
. ~/.config/cailoxo/prompt.ps1
```

## Updating

After upgrading `cailoxo`, regenerate scripts:

```sh
cailoxo generate --shell zsh --output ~/.config/cailoxo/prompt.zsh
cailoxo generate --shell nu --output ~/.config/cailoxo/prompt.nu
cailoxo generate --shell pwsh --output ~/.config/cailoxo/prompt.ps1
```

If you use a custom config path, include it every time you regenerate:

```sh
cailoxo generate --shell zsh --config ~/.config/cailoxo/theme.toml --output ~/.config/cailoxo/prompt.zsh
```

Restart shell sessions or source the generated script again.
