# Shell Support

`cailoxo` generates native scripts for each shell. Shell behavior is similar, but not identical, because prompt APIs differ.

## zsh

Supported:

- adaptive two-line prompt
- transient prompt
- terminal resize repaint
- async remote fetch with changed-only repaint
- OSC8 path and Git links
- OSC7 current-directory metadata

Notes:

- Generated script uses `prompt_subst`.
- Non-printing sequences are wrapped in `%{...%}`.
- Async fetch uses ZLE file descriptor callbacks.

## Nushell

Supported:

- adaptive two-line prompt
- transient prompt through prompt environment variables
- async remote fetch picked up on next prompt render
- Windows path display separators

Limitations:

- Reedline does not re-run `PROMPT_COMMAND` for resize while input is active.
- Reedline does not expose OSC8 links from prompt strings reliably, so generated Nu prompts ignore path/Git `url` settings and path `osc7`.
- Fetch completion does not repaint current prompt; next prompt render picks up fetched refs.

## PowerShell

Supported:

- adaptive two-line prompt
- transient prompt through PSReadLine Enter handler
- async remote fetch with changed-only repaint attempt
- OSC8 path and Git links
- OSC7 current-directory metadata
- Windows path display separators

Notes:

- Generated script uses native `function prompt`.
- `Set-PSReadLineOption -ExtraPromptLineCount` helps multiline prompt clearing.
- Repaint uses `PSConsoleReadLine.InvokePrompt()` when available.

## Feature Matrix

| Feature | zsh | nu | pwsh |
| --- | --- | --- | --- |
| Adaptive path | yes | yes | yes |
| Git status | yes | yes | yes |
| Async fetch | yes | yes | yes |
| Fetch repaint | yes | next prompt | yes |
| Transient prompt | yes | yes | yes |
| Resize repaint | yes | limited | shell-dependent |
| Path OSC8 link | yes | no | yes |
| Git OSC8 link | yes | no | yes |
| OSC7 CWD metadata | yes | no | yes |
