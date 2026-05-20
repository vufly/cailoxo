# Troubleshooting

## Prompt Does Not Change

Regenerate the script after editing `cailoxo.toml`:

```sh
cailoxo generate --shell zsh --output ~/.config/cailoxo/prompt.zsh
```

Then start a new shell or source the generated script again.

## Icons Show As Boxes

Install and select a Nerd Font in terminal settings. The default config uses Nerd Font glyphs for OS, folder, Git, and powerline separators.

## Git Status Looks Stale

Ahead/behind counts use local refs. Enable background fetch if you want remote refs refreshed:

```toml
[line.span.settings]
fetch_remote = true
fetch_remote_interval_ms = 60000
fetch_remote_timeout_ms = 5000
```

Fetch behavior differs by shell:

- `zsh`: repaints when fetched refs change.
- `pwsh`: attempts repaint when fetched refs change.
- `nu`: updates on next prompt render.

## Clean Git Repo Has Extra Space

Use conditional status spacing in Git template:

```toml
template = " {{ upstream_icon }}{{ branch_icon }}{{ branch }}{{ if status }} {{ status }}{{ end }} "
```

## Path Or Git Links Do Not Work

Links require terminal OSC8 support. zsh and PowerShell generated scripts emit OSC8 links when enabled:

```toml
[line.span.settings]
url = true
```

Nushell/Reedline does not expose OSC8 links from prompt strings reliably, so generated Nu prompts ignore `url` settings.

## Nu Command Output Disappears After Transient Prompt

Use a generated prompt from `v0.1.2` or newer. Nu prompt rendering is split between `PROMPT_COMMAND` and `PROMPT_INDICATOR` to avoid Reedline clearing command output incorrectly.

## PowerShell Prompt Repaints Noticeably

PowerShell repaint uses `PSConsoleReadLine.InvokePrompt()`, which can be visible. `cailoxo` only requests repaint when background fetch changes upstream refs.

## zsh Syntax Check

```sh
zsh -n output/prompt.zsh
```

## Nu Smoke Test

```sh
nu --no-config-file --commands 'source output/prompt.nu; print (do $env.PROMPT_COMMAND); print (do $env.PROMPT_INDICATOR)'
```

## PowerShell Smoke Test

```sh
pwsh -NoLogo -NoProfile -Command '. ./output/prompt.ps1; prompt | Out-String'
```
