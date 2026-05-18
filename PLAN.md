# Plan

`cailoxo` generates native prompt scripts from TOML. Current MVP targets `zsh` and Nushell with no prompt framework required at runtime.

## Current MVP

Implemented:

- Rust CLI and config parser.
- `cargo run -- generate --shell zsh` writes `output/prompt.zsh` by default.
- `cargo run -- generate --shell nu` writes `output/prompt.nu` by default.
- TOML schema for lines, spans, colors, edges, transient prompt, and Git settings.
- Native zsh prompt renderer.
- Native Nushell prompt renderer.
- Adaptive path truncation on prompt render.
- Local-only Git branch/status collection.
- Local ahead/behind counts from existing upstream refs.
- Built-in icon dictionaries from `defaults/icons.toml`.
- OMP-style template decoration tags.
- Path `edge_format` and `gitdir_format` with OMP-style `%s` format strings.
- zsh transient prompt.
- Nushell transient prompt variables.

Deferred:

- PowerShell generator.
- Right prompt.
- Async rendering.
- Network fetches.
- Full oh-my-posh or Starship compatibility.
- Theme marketplace.
- Arbitrary custom spans.

## Prompt Shape

Current prompt is a two-line left prompt:

```text
<os> <path> <git>
   <char>
```

Example:

```text
   ~/repos/cailoxo   main ?8 | !1 
   
```

Transient prompt keeps only prompt character:

```text
 
```

## Config Shape

Primary config is `cailoxo.toml`.

Current span types:

- `os`: platform or distro icon from `defaults/icons.toml`.
- `path`: adaptive current directory display.
- `git`: local Git branch, upstream icon, and status.
- `text`: prompt character span.

Example Git span:

```toml
[[line.span]]
type = "git"
template = " {{ upstream_icon }}{{ branch_icon }}{{ branch }} {{ status }} "
foreground = "0"
background = "2"
dirty_background = "3"
separator = ""
invert_separator = false

[line.span.settings]
source = "cli"
fetch_status = true
fetch_remote = false
fetch_upstream_icon = false
show_branch_status = false
show_stash_count = true
icon_set = "minimal"
separator = " |"

[line.span.settings.status]
ahead = "{{status_icon}}{{ count }}"
behind = "{{status_icon}}{{ count }}"
conflicted = "{{status_icon}}{{ count }}"
untracked = "{{status_icon}}{{ count }}"
modified = "{{status_icon}}{{ count }}"
staged = "{{status_icon}}{{ count }}"
renamed = "{{status_icon}}{{ count }}"
deleted = "{{status_icon}}{{ count }}"
stashed = "{{status_icon}}{{ count }}"
```

## Edges

There is no `style` field. Span visual behavior comes from edge fields:

- no `separator`, `head`, or `tail`: plain span.
- `separator`: powerline-like connection.
- `head` or `tail`: capped/diamond-like span.
- `invert_separator`, `invert_head`, `invert_tail`: swap edge foreground/background.
- `span[i].separator` and `span[i + 1].head` is invalid because both own same boundary.

## Templates

Template syntax:

```text
{{ variable }}
{{ if variable }}...{{ else }}...{{ end }}
```

Decoration tags use OMP-style markup and are rendered by zsh/Nu generators:

```text
<b>{{ branch }}</b>
<b><i>%s</i></b>
```

Supported tags:

- `<b>` bold
- `<i>` italic
- `<u>` underline
- `<o>` overline
- `<s>` strikethrough
- `<d>` dim
- `<f>` blink
- `<r>` reverse

Template-like format settings also use decoration tags:

- `edge_format = "<b>%s</b>"`
- `gitdir_format = "<b><i>%s</i></b>"`

## Icons

Built-in dictionaries live in `defaults/icons.toml` and are embedded into generated scripts:

- `os.nerdfont`: platform and distro icons.
- `git_branch.nerdfont`: branch/worktree icons.
- `git_upstream.nerdfont`: upstream provider icons.
- `git_status.minimal`: compact status symbols.
- `git_status.nerdfont`: Nerd Font status symbols.

Current config uses:

- OS icons from `os.nerdfont`.
- `branch_icon` from `git_branch.nerdfont.git` unless explicitly overridden.
- `upstream_icon` from `git_upstream.nerdfont` when `fetch_upstream_icon = true`.
- `status_icon` from `git_status.<icon_set>` inside `[line.span.settings.status]` templates.

## Path

Path span gets remaining width after fixed-width spans are measured:

```text
path_budget = terminal_width - visible_width(os) - visible_width(git) - fixed_edges_and_padding
```

Current renderers try:

- full home-shortened path.
- `~/…/<parent>/<current>`.
- `…/<current>`.

Width math strips decoration tags and ANSI escapes before counting.

`edge_format` applies to first and last visible path parts. `gitdir_format` applies to visible Git repo root folder.

## Git

Git data is local-only:

- branch or detached HEAD short SHA.
- upstream provider from local Git config only.
- ahead/behind from existing upstream refs only.
- conflicted/untracked/modified/staged/renamed/deleted counts from `git status --porcelain=v1`.
- stash count from local stash list.
- no `git fetch`.
- no network access.

Status kinds:

- `ahead`
- `behind`
- `conflicted`
- `untracked`
- `modified`
- `staged`
- `renamed`
- `deleted`
- `stashed`

## Shell Notes

zsh:

- Uses `prompt_subst`.
- Wraps non-printing ANSI in `%{...%}`.
- Repaints on `WINCH` through guarded ZLE reset.
- Supports transient prompt through `zle-line-finish`.

Nushell:

- Uses prompt environment variables and closures.
- Recomputes prompt on prompt render.
- Terminal resize while input is already active repaints existing prompt string; Nushell/reedline does not re-run `PROMPT_COMMAND` mid-input.

## Validation

Current validation commands:

```sh
cargo fmt
cargo test
cargo clippy -- -D warnings
cargo run -- generate --shell zsh
cargo run -- generate --shell nu
zsh -n output/prompt.zsh
nu --no-config-file --commands 'source output/prompt.nu; $env.PROMPT_COMMAND | do $in'
```

## Next Work

- Improve Nu/zsh truncation parity for edge cases.
- Add tests around icon-set expansion and `{{ status_icon }}`.
- Add configurable built-in icon overrides.
- Add PowerShell generator after zsh/Nu behavior stabilizes.
- Expand docs with generated output examples.
