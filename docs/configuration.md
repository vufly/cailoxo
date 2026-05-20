# Configuration

`cailoxo` reads `cailoxo.toml` and generates native shell scripts. MVP supports `zsh`, Nushell, and PowerShell.

## Generate

```sh
cailoxo generate --shell zsh
cailoxo generate --shell nu
cailoxo generate --shell pwsh
```

Default outputs:

- `output/prompt.zsh`
- `output/prompt.nu`
- `output/prompt.ps1`

Use `--output` to write somewhere else.

Use `--config` to read a custom TOML file instead of `cailoxo.toml`:

```sh
cailoxo generate --shell zsh --config ~/.config/cailoxo/theme.toml --output ~/.config/cailoxo/prompt.zsh
```

Source generated scripts from shell startup files.

```zsh
source ~/.config/cailoxo/prompt.zsh
```

```nu
source ~/.config/cailoxo/prompt.nu
```

```powershell
. ~/.config/cailoxo/prompt.ps1
```

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

## Structure

Top-level config:

```toml
version = 1
final_space = true

[transient]
enabled = true
template = " "
```

Prompt lines use `[[line]]`. Inline prompt units use `[[line.span]]`.

```toml
[[line]]
newline = true

[[line.span]]
type = "path"
template = " {{ path }} "
foreground = "0"
background = "4"
separator = ""
```

Supported span types for MVP:

- `os`
- `path`
- `git`
- `text`

## Edges

There is no `style` field. Visual behavior comes from edge fields.

```toml
separator = ""
invert_separator = false

head = ""
tail = ""
invert_head = false
invert_tail = false
```

Rules:

- no edge fields means plain span
- `separator` means powerline-like connection
- `head` or `tail` means capped span
- `span[i].separator` and `span[i + 1].head` is invalid because both own same boundary

## Colors

Colors support terminal palette indexes, names, and truecolor hex values.

```toml
foreground = "0"
background = "4"

foreground = "black"
background = "blue"

foreground = "#cdd6f4"
background = "#1e1e2e"
```

## Path

Path span supports adaptive truncation.

```toml
[[line.span]]
type = "path"
template = " {{ if home }} {{ else }} {{ end }}{{ path }} "
foreground = "0"
background = "4"
separator = ""

[line.span.settings]
home_icon = "~"
folder_icon = "…"
mode = "adaptive"
min_dirs = 1
edge_format = "<b>%s</b>"
gitdir_format = "<b><i>%s</i></b>"
url = true
osc7 = true
```

The generated prompt recalculates on each render and uses current terminal width.

`edge_format` formats the first and last visible path parts. `gitdir_format` formats the Git repository root folder when it is visible in the path. Both use OMP-style `%s` format strings and support the same decoration tags as templates.

`url = true` wraps the rendered path span in an OSC8 `file://` hyperlink, so supported terminals can open the folder from the prompt. `osc7 = true` emits OSC7 current-directory metadata before the prompt for terminals that track shell CWD.

Nushell/Reedline does not expose OSC8 links from prompt strings reliably. Generated Nu prompts ignore `url` and `osc7` for path spans.

## Git

Git span uses local repo state by default. It can also refresh upstream refs asynchronously when `fetch_remote = true`.

```toml
[[line.span]]
type = "git"
template = " {{ upstream_icon }}{{ branch_icon }}{{ branch }}{{ if status }} {{ status }}{{ end }} "
foreground = "0"
background = "2"
dirty_background = "3"
separator = ""

[line.span.settings]
fetch_status = true
fetch_remote = false
fetch_remote_interval_ms = 60000
fetch_remote_timeout_ms = 5000
fetch_upstream_icon = false
show_branch_status = false
show_stash_count = true
icon_set = "minimal"
separator = " |"
url = true
```

Ahead and behind counts use existing local upstream refs. With `fetch_remote = true`, the prompt still renders immediately from local refs, then starts a throttled background `git fetch --quiet --no-tags <remote>`.

Refresh behavior differs by shell:

- `zsh`: repaints the current prompt when fetch completes.
- `pwsh`: attempts to repaint the current prompt with `PSConsoleReadLine.InvokePrompt()` when fetch completes.
- `nu`: picks up fetched refs on the next prompt render.

`fetch_remote_interval_ms` throttles fetch starts per repository/remote. `fetch_remote_timeout_ms` limits each background fetch in zsh and PowerShell when supported.
Git `branch_icon`, `upstream_icon`, and `status` symbols come from `defaults/icons.toml`. `icon_set` selects `git_status.<set>`; default config uses `minimal`.

`url = true` wraps the rendered Git span in an OSC8 hyperlink to the upstream repository. SSH remotes such as `git@github.com:user/repo.git` are converted to browser URLs such as `https://github.com/user/repo`.

Nushell/Reedline does not expose OSC8 links from prompt strings reliably. Generated Nu prompts ignore `url` for Git spans.

## Templates

Templates use documented span variables.

```text
{{ variable }}
{{ if variable }}...{{ else }}...{{ end }}
```

See `docs/template-variables.md`.
