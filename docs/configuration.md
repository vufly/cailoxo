# Configuration

`cailoxo` reads `cailoxo.toml` and generates native shell scripts. MVP supports `zsh` and Nushell.

## Generate

```sh
cailoxo generate --shell zsh
cailoxo generate --shell nu
```

Default outputs:

- `output/prompt.zsh`
- `output/prompt.nu`

Use `--output` to write somewhere else.

Source generated scripts from shell startup files.

```zsh
source ~/.config/cailoxo/prompt.zsh
```

```nu
source ~/.config/cailoxo/prompt.nu
```

Quick interactive tests:

```sh
cargo run -- generate --shell zsh; tmp=$(mktemp -d); printf 'source %q/output/prompt.zsh\n' "$PWD" > "$tmp/.zshrc"; ZDOTDIR="$tmp" zsh -i
```

```sh
cargo run -- generate --shell nu; nu --no-config-file --execute 'source output/prompt.nu'
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
```

The generated prompt recalculates on each render and uses current terminal width.

`edge_format` formats the first and last visible path parts. `gitdir_format` formats the Git repository root folder when it is visible in the path. Both use OMP-style `%s` format strings and support the same decoration tags as templates.

## Git

Git span uses local repo state only. It does not fetch.

```toml
[[line.span]]
type = "git"
template = " {{ upstream_icon }}{{ branch_icon }}{{ branch }} {{ status }} "
foreground = "0"
background = "2"
dirty_background = "3"
separator = ""

[line.span.settings]
fetch_status = true
fetch_remote = false
fetch_upstream_icon = false
show_branch_status = false
show_stash_count = true
icon_set = "minimal"
separator = " |"
```

Ahead and behind counts use existing local upstream refs only.
Git `branch_icon`, `upstream_icon`, and `status` symbols come from `defaults/icons.toml`. `icon_set` selects `git_status.<set>`; default config uses `minimal`.

## Templates

Templates use documented span variables.

```text
{{ variable }}
{{ if variable }}...{{ else }}...{{ end }}
```

See `docs/template-variables.md`.
