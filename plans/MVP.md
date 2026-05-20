# MVP

MVP status: complete.

`cailoxo` generates native prompt scripts from TOML for `zsh`, Nushell, and PowerShell. Generated scripts render the prompt directly in each shell; no prompt framework is required at runtime.

## Scope

MVP prompt shape:

```text
<os> <path> <git>
   <char>
```

Transient prompt shape:

```text
<char>
```

## Delivered

- Rust CLI and TOML config parser.
- `generate` command with shell IDs `zsh`, `nu`, and `pwsh`.
- Default output paths:
  - `output/prompt.zsh`
  - `output/prompt.nu`
  - `output/prompt.ps1`
- Config schema for lines, spans, colors, edges, transient prompt, and Git settings.
- Span types:
  - `os`
  - `path`
  - `git`
  - `text`
- Native zsh renderer.
- Native Nushell renderer.
- Native PowerShell renderer.
- Adaptive path truncation on prompt render.
- Path `edge_format` and `gitdir_format` with OMP-style `%s` format strings.
- OMP-style decoration tags:
  - `<b>` bold
  - `<i>` italic
  - `<u>` underline
  - `<o>` overline
  - `<s>` strikethrough
  - `<d>` dim
  - `<f>` blink
  - `<r>` reverse
- Local Git branch/status collection.
- Ahead/behind counts from existing local upstream refs.
- Optional throttled background `git fetch` outside prompt rendering.
- Built-in icons embedded from `defaults/icons.toml`.
- zsh transient prompt through `zle-line-finish`.
- Nushell transient prompt through prompt environment variables.
- PowerShell transient prompt through PSReadLine Enter handler.
- zsh/pwsh OSC8 links for path and Git spans.
- zsh/pwsh OSC7 current-directory metadata for path spans.
- Documentation for config and template variables.
- Tag-driven release workflow for GitHub assets and crates.io.

## Key Decisions

- Rust generator, native shell output.
- TOML config, not JSON/YAML.
- No runtime helper in prompt loop.
- Native shell IDs use exact values: `zsh`, `nu`, `pwsh`.
- Edges use `head`, `tail`, `separator`, and invert flags; no `style` field.
- `span[i].separator` plus `span[i + 1].head` is invalid.
- Git status separator is respected exactly from config.
- Clean Git status does not leave extra branch spacing.
- Async fetch is optional and never blocks first prompt render.
- Nu OSC8 prompt links are not supported because Reedline does not expose prompt-string OSC8 links reliably.

## Git Behavior

Git span supports:

- branch or detached HEAD short SHA
- upstream provider icon from remote URL
- ahead/behind from local upstream refs
- conflicted count
- untracked count
- modified count
- staged count
- renamed count
- deleted count
- stashed count

With `fetch_remote = true`, generated prompts start a throttled background fetch. zsh and PowerShell repaint only when fetched refs change. Nu picks up fetched refs on next prompt render.

## Shell Notes

zsh:

- Uses `prompt_subst`.
- Wraps non-printing sequences in `%{...%}`.
- Repaints on terminal resize.
- Uses `zle -F` for async fetch completion.

Nushell:

- Uses prompt closures in environment variables.
- Prompt command and prompt indicator are split to avoid transient prompt output loss.
- Resize mid-input is limited by Reedline behavior.
- OSC8 prompt links are intentionally disabled.

PowerShell:

- Uses native `function prompt`.
- Uses PSReadLine extra prompt line count.
- Uses PSReadLine Enter handler for transient prompt.
- Attempts repaint with `PSConsoleReadLine.InvokePrompt()` after changed fetches.

## Release History

- `v0.1.0`: initial native prompt generator release.
- `v0.1.1`: async fetch and cross-shell path improvements.
- `v0.1.2`: Nu transient fix and changed-only fetch repaint release.

## Validation

MVP validation command set:

```sh
cargo fmt --check
cargo test --locked
cargo clippy --locked -- -D warnings
cargo package --locked --allow-dirty
cargo run -- generate --shell zsh
cargo run -- generate --shell nu
cargo run -- generate --shell pwsh
zsh -n output/prompt.zsh
nu --no-config-file --commands 'source output/prompt.nu; print (do $env.PROMPT_COMMAND); print (do $env.PROMPT_INDICATOR)'
pwsh -NoLogo -NoProfile -Command '. ./output/prompt.ps1; prompt | Out-String'
```
